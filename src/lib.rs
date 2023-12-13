use prover::shared_state::generate_proof;
use tokio::sync:: mpsc;
use tracing::error;
use lazy_static::lazy_static;
use tokio::sync::Mutex;
use std::sync::Arc;

use tokio::task::JoinHandle;


lazy_static! {
    pub static ref CANCEL_FLAG: Arc<Mutex<u8>> = {
        Arc::new(Mutex::new(0))
    };

}

/// test function
pub fn test() -> String {
    "hello world".to_string()
}

/// init_env function is used to init the computation setup env,like CPU/GPU initialization
pub fn init_env() {  
    todo!()  //TBD
}

/// gen_task_proof function is used to generate one task proof 
/// if any error occur,it will return "error"
pub async fn gen_task_proof(task_id:u64,task_content:String,l2_url:String,l1_url:String) -> String { 
    let task_vec: Vec<&str> = task_content.split("#").collect(); //Parse the task content
    if task_vec.len() != 14{
        error!("{} task parameter error,ignore it",task_id.clone());
        return "error".to_string()
    }
    let prover_address=task_vec[0].to_string();
    let l1_signal_service=task_vec[1].to_string();
    let l2_signal_service=task_vec[2].to_string();
    let taiko_12=task_vec[3].to_string();
    let meta_hash=task_vec[4].to_string();
    let blockhash=task_vec[5].to_string();
    let parenthash=task_vec[6].to_string();
    let signalroot=task_vec[7].to_string();
    let graffiti=task_vec[8].to_string();

    let gasused=task_vec[9].to_string().parse::<u64>().unwrap();
    let parentgasused=task_vec[10].parse::<u64>().unwrap();
    let blockmaxgasimit=task_vec[11].parse::<u64>().unwrap();
    let maxtransactionsperblock=task_vec[12].parse::<u64>().unwrap();
    let maxbytespertxlist=task_vec[13].parse::<u64>().unwrap();

    let agg_proof_result = match generate_proof(
        l2_url,
        task_id,
        prover_address.clone(),
        l1_signal_service.clone(),
        l2_signal_service.clone(),
        taiko_12.clone(),
        meta_hash.clone(),
        blockhash.clone(),
        parenthash.clone(),
        signalroot.clone(),
        graffiti.clone(),
        gasused,
        parentgasused,
        blockmaxgasimit,
        maxtransactionsperblock,
        maxbytespertxlist).await{
            Ok(r) => r,
            Err(_) => {
                return "error".to_string()
            },
        };
    let mut proofoutput =String::from("");
    for var in &agg_proof_result.instance{
        proofoutput=format!("{}#{}",proofoutput,var.to_string())
    }
    format!("{}#{}",proofoutput,agg_proof_result.proof)  
}

/// gen_task_proof function is used to generate one task proof 
/// if any error occur,it will return "error"
pub async fn gen_task_proof_spawn(task_id:u64,task_content:String,l2_url:String,l1_url:String) -> String { 
    let task_vec: Vec<&str> = task_content.split("#").collect(); //Parse the task content
    if task_vec.len() != 14{
        error!("{} task parameter error,ignore it",task_id.clone());
        return "error".to_string()
    }
    let prover_address=task_vec[0].to_string();
    let l1_signal_service=task_vec[1].to_string();
    let l2_signal_service=task_vec[2].to_string();
    let taiko_12=task_vec[3].to_string();
    let meta_hash=task_vec[4].to_string();
    let blockhash=task_vec[5].to_string();
    let parenthash=task_vec[6].to_string();
    let signalroot=task_vec[7].to_string();
    let graffiti=task_vec[8].to_string();

    let gasused=task_vec[9].to_string().parse::<u64>().unwrap();
    let parentgasused=task_vec[10].parse::<u64>().unwrap();
    let blockmaxgasimit=task_vec[11].parse::<u64>().unwrap();
    let maxtransactionsperblock=task_vec[12].parse::<u64>().unwrap();
    let maxbytespertxlist=task_vec[13].parse::<u64>().unwrap();

    let (tx, mut rx) = mpsc::channel(100);

    let tx2=tx.clone();

    tokio::spawn(async move {
        loop{
            let current = CANCEL_FLAG.clone();
            let need_cancel = current.lock().await;
            if *need_cancel==1 {
                let _ = tx2.send(1).await;
                break;
            }
        }
    });

    tokio::select! {
        val = generate_proof(
            l2_url,
            task_id,
            prover_address.clone(),
            l1_signal_service.clone(),
            l2_signal_service.clone(),
            taiko_12.clone(),
            meta_hash.clone(),
            blockhash.clone(),
            parenthash.clone(),
            signalroot.clone(),
            graffiti.clone(),
            gasused,
            parentgasused,
            blockmaxgasimit,
            maxtransactionsperblock,
            maxbytespertxlist) => {
            let proof_res=match val {
                Ok(r) => r,
                Err(_) => {
                    return "error".to_string()
                },
            };
            let mut proofoutput =String::from("");
            for var in &proof_res.instance{
                proofoutput=format!("{}#{}",proofoutput,var.to_string())
            }
            format!("{}#{}",proofoutput,proof_res.proof) 
        }
        _ = rx.recv() => {
            return "error".to_string()
        }
    }

}

/// cancel_task_proof function is used to cancel one proof task,
/// clear the memory and CPU/GPU resource if needed
pub async fn cancel_task_proof(_task_id:u64)  { 
    let current = CANCEL_FLAG.clone();
    let mut need_cancel = current.lock().await;
    *need_cancel = 1;

    todo!() //else to do,clear GPU/CPU/MEMORY
}



#[cfg(test)]
mod tests {
    use std::time::Instant;

    use super::*;

    #[tokio::test]
    async fn it_works() {
        let subscriber = tracing_subscriber::fmt::Subscriber::builder()
        .with_max_level(tracing::Level::INFO)
        .finish();
        tracing::subscriber::set_global_default(subscriber).expect("unable to set global default subscriber");

        let l2_url="https://rpc.jolnir.taiko.xyz/".to_string();
        let task_id = 20865 as u64;
        let task_content = "94061Fd498291Ff1F1b8C0d1a94e2EDC2a0A2f9D#cD5e2bebd3DfE46e4BF96aE2ac7B89B22cc6a982#1000777700000000000000000000000000000007#1000777700000000000000000000000000000001#322e41c411a8223cce152999b30ee00b8f29dc5e62e02f43e0dc7a77aa862fa8#c73622fae1fbc1d1d9e4a9b7bbdb6733595c1c98a2470ea59ca3b9fee9ba3894#afcb03ea890fb2d5ba0042fcda321d8879687fb87a8d68b8ef4417dbc86754b0#9cc94396d73d6c51d8185249a1bcc7c55c87b3d6b67ce72600cfc8448dadc007#0000000000000000000000000000000000000000000000000000000000000000#1241987#328517#8000000#0#120000".to_string();
        let time_started = Instant::now();
        let result = gen_task_proof_spawn(task_id,task_content,l2_url.clone(),l2_url).await;
        println!("result is {},time consume is {}",result,Instant::now().duration_since(time_started).as_secs());
    }
}
