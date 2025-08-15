use servidor_http::Error;
use servidor_http::dispatcher;
use servidor_http::dispatcher::Dispatcher;

fn dispatcher_fun() -> Result<(), Error> {
    Ok(())
}

#[test]
fn test_single_thread_dispatcher() {
    let st = dispatcher::SingleThreadDispatcher;

    match st.dispatch(Box::new(dispatcher_fun)) {
        Ok(()) => {},
        _ => {
            panic!("Couldn't dispatch job");
        }
    }; 
}

#[test]
fn test_fork_dispatcher() {
    let fd = dispatcher::ForkDispatcher;

    match fd.dispatch(Box::new(dispatcher_fun)) {
        Ok(()) => {},
        _ => {
            panic!("Couldn't dispatch job");
        }
    };
}

#[test]
fn test_thread_pool_dispatcher() {
    let mut tpd =  dispatcher::thread_pool_dispatcher::ThreadPoolDispatcher::new(5);

    tpd.run();

    for _ in 1..100 {
        match tpd.dispatch(Box::new(dispatcher_fun)) {
            Ok(()) => {},
            _ => {
                panic!("Couldn't dispatch job");
            }
        };
    }
}

