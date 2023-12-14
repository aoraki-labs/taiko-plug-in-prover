## Introduction
    One taiko-plug-in-prover lib to support taiko project prover

## lib interface

### init_env
    init_env function is used to init the computation setup env,like CPU/GPU initialization
```
   pub fn init_env() {  
   }
```

### gen_task_proof
    gen_task_proof function is used to generate one task proof,if any error occurs,it will return "error"
```
    pub async fn gen_task_proof(task_id:String,task_content:String,l2_url:String,l1_url:String) -> String { 
    }
    task_id : the taiko task ids
    task_content : the task content,put together with "#" if there are several parameters
    l2_url:String : taiko L2 network http_url endpoint
    l1_url:String : taiko L1 network http_url endpoint
```

### cancel_task_proof
    cancel_task_proof function is used to cancel one proof task,clear the memory and CPU/GPU resource if needed
```
    pub async fn cancel_task_proof(_task_id:u64)  { 
    }
    task_id : the taiko task id
```

## run test
```
    wget https://storage.googleapis.com/zkevm-circuits-keys/kzg_bn254_22.srs -P ./
    cargo test --release -- --nocapture
```

## lib
    if you want to integrate the compiled lib directly,refer to the lib/dylib file : 
        libtaiko_plug_in_prover.rlib
        libtaiko_prover.so
