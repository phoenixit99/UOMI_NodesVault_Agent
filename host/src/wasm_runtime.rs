use crate::api_service::{call_service_api, get_file_from_cid, get_file};
use crate::blockchain_service::get_wallet_balance;
use crate::utils::generate_wasm_data;

pub fn run_wasm(wasm: Vec<u8>, input: Vec<u8>) -> Result<Vec<u8>, wasmi::Error> {
    let engine = wasmi::Engine::default();
    let module = wasmi::Module::new(&engine, &wasm[..])?;
    let mut store = wasmi::Store::new(&engine, input);

    type HostState = Vec<u8>;

    let console_log = wasmi::Func::wrap(
        &mut store,
        | caller: wasmi::Caller<'_, HostState>, ptr: i32, len: i32| {
            let memory = caller
                .get_export("memory")
                .and_then(wasmi::Extern::into_memory)
                .expect("Failed to get memory export");

            let mut buffer = vec![0u8; len as usize];
            memory
                .read(&caller, ptr as usize, &mut buffer)
                .expect("Failed to read memory");

            if let Ok(message) = String::from_utf8(buffer) {
                println!("[WASM] {}", message);
            }
        }
    );
    let get_input_data = wasmi::Func::wrap(
        &mut store,
        move |mut caller: wasmi::Caller<'_, HostState>, ptr: i32, _len: i32| {
            let data_to_write = generate_wasm_data(caller.data().clone());
        
            let memory = caller
                .get_export("memory")
                .and_then(wasmi::Extern::into_memory)
                .expect("Failed to get memory export");

            memory
                .write(&mut caller, ptr as usize, &data_to_write)
                .expect("Failed to write memory");
        }
    );

    let set_output = wasmi::Func::wrap(
        &mut store,
        |mut caller: wasmi::Caller<'_, HostState>, ptr: i32, len: i32| {
            let memory = caller
                .get_export("memory")
                .and_then(wasmi::Extern::into_memory)
                .expect("Failed to get memory export");

            let mut buffer = vec![0u8; len as usize];
            memory
                .read(&caller, ptr as usize, &mut buffer)
                .expect("Failed to read memory");

            *caller.data_mut() = buffer;
        }
    );

    let call_ai = wasmi::Func::wrap(
        &mut store,
        move |mut caller: wasmi::Caller<'_, HostState>, model: i32, ptr: i32, len: i32, output_ptr: i32, _: i32| {
            let memory = caller
                .get_export("memory")
                .and_then(wasmi::Extern::into_memory)
                .expect("Failed to get memory export");

            let mut input_for_service = vec![0u8; len as usize];

            memory
                .read(&caller, ptr as usize, &mut input_for_service)
                .expect("Failed to read memory");

            // Convert model from i32 to AiModelKey type
            let service_output_text = async {
                call_service_api(model, input_for_service).unwrap()
            };

            let service_output = futures::executor::block_on(service_output_text);
            
            let service_output = generate_wasm_data(service_output);

            let memory = caller
            .get_export("memory")
            .and_then(wasmi::Extern::into_memory)
            .expect("Failed to get memory export");

        memory
            .write(&mut caller, output_ptr as usize, &service_output)
            .expect("Failed to write memory");
        }
    );

    let get_cid_file = wasmi::Func::wrap(
        &mut store,
        move |mut caller: wasmi::Caller<'_, HostState>, ptr: i32, len: i32, output_ptr: i32, _: i32| {
            let memory = caller
                .get_export("memory")
                .and_then(wasmi::Extern::into_memory)
                .expect("Failed to get memory export");
    
            let mut buffer = vec![0u8; len as usize];
    
            memory
                .read(&caller, ptr as usize, &mut buffer)
                .expect("Failed to read memory");
    
            let cid = String::from_utf8(buffer).unwrap();
    
            // Create a new tokio runtime for this thread
            let rt = tokio::runtime::Runtime::new().unwrap();
            let file_bytes = rt.block_on(async {
                get_file_from_cid(&cid).await.unwrap()
            });
            
            let data_to_write = generate_wasm_data(file_bytes);
    
            let memory = caller
                .get_export("memory")
                .and_then(wasmi::Extern::into_memory)
                .expect("Failed to get memory export");
    
            memory
                .write(&mut caller, output_ptr as usize, &data_to_write)
                .expect("Failed to write memory");
        }
    );

    let get_input_file = wasmi::Func::wrap(
        &mut store,
        move |mut caller: wasmi::Caller<'_, HostState>, ptr: i32, _len: i32| {

            let file = get_file().unwrap();
            let data_to_write = generate_wasm_data(file);

            let memory = caller
                .get_export("memory")
                .and_then(wasmi::Extern::into_memory)
                .expect("Failed to get memory export");

            memory
                .write(&mut caller, ptr as usize, &data_to_write)
                .expect("Failed to write memory");
        }
    );

    let call_blockchain = wasmi::Func::wrap(
        &mut store,
        move |mut caller: wasmi::Caller<'_, HostState>, ptr: i32, len: i32, output_ptr: i32, _: i32| -> i32 {
            let memory = caller
                .get_export("memory")
                .and_then(wasmi::Extern::into_memory)
                .expect("Failed to get memory export");

            let mut buffer = vec![0u8; len as usize];
            memory
                .read(&caller, ptr as usize, &mut buffer)
                .expect("Failed to read memory");

            let request = String::from_utf8(buffer).unwrap();
            let request_json: serde_json::Value = serde_json::from_str(&request).unwrap();

            if let Some(address) = request_json.get("address").and_then(|v| v.as_str()) {
                let rt = tokio::runtime::Runtime::new().unwrap();
                match rt.block_on(async { get_wallet_balance(address).await }) {
                    Ok(balance) => {
                        let balance_json = serde_json::to_vec(&balance).unwrap();
                        let data_to_write = generate_wasm_data(balance_json);
                        
                        memory
                            .write(&mut caller, output_ptr as usize, &data_to_write)
                            .expect("Failed to write memory");
                        0 // Success
                    }
                    Err(e) => {
                        println!("Blockchain service error: {:?}", e);
                        1 // Error
                    }
                }
            } else {
                println!("Invalid blockchain request format");
                1 // Error
            }
        }
    );

    let mut linker = wasmi::Linker::new(&engine);
    linker.define("env", "get_input_data", get_input_data)?;
    linker.define("env", "set_output", set_output)?;
    linker.define("env", "call_ai", call_ai)?;
    linker.define("env", "console_log", console_log)?;
    linker.define("env", "get_cid_file", get_cid_file)?;
    linker.define("env", "get_input_file", get_input_file)?;
    linker.define("env", "call_blockchain", call_blockchain)?;

    let instance = linker.instantiate(&mut store, &module)?.start(&mut store)?;
    let wasm_run = instance.get_typed_func::<(), ()>(&store, "run")?;

    wasm_run.call(&mut store, ())?;

    Ok(store.into_data())
}
