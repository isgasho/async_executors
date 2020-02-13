use
{
	futures_util :: { task::{ LocalSpawnExt, SpawnError }         } ,
	crate        :: { LocalSpawnHandle                            } ,
	std          :: { pin::Pin, future::Future, sync::Arc, rc::Rc } ,
};


/// Object safe version of [crate::SpawnHandle]. This allows you to take it as a param
/// and store it. It incurs some overhead, since the future needs to be boxed and executors
/// will box it again to queue it.
///
/// It also implies you have to choose an Out type.
//
pub trait LocalSpawnHandleOs<Out: 'static> : LocalSpawnExt
{
	/// Spawn a future and return a RemoteHandle that can be awaited for the output of the future.
	//
	fn spawn_handle_local_os( &self, future: Pin<Box< dyn Future<Output = Out> >> ) -> Result<crate::JoinHandle<Out>, SpawnError>;
}



impl<T, Out> LocalSpawnHandleOs<Out> for Arc<T> where T: LocalSpawnHandleOs<Out>, Out: 'static
{
	fn spawn_handle_local_os( &self, future: Pin<Box< dyn Future<Output = Out> >> ) -> Result<crate::JoinHandle<Out>, SpawnError>
	{
		(**self).spawn_handle_local_os( future )
	}
}


impl<T, Out> LocalSpawnHandleOs<Out> for Rc<T> where T: LocalSpawnHandleOs<Out>, Out: 'static
{
	fn spawn_handle_local_os( &self, future: Pin<Box< dyn Future<Output = Out> >> ) -> Result<crate::JoinHandle<Out>, SpawnError>
	{
		(**self).spawn_handle_local_os( future )
	}
}


impl<T, Out> LocalSpawnHandleOs<Out> for &T where T: LocalSpawnHandleOs<Out>, Out: 'static
{
	fn spawn_handle_local_os( &self, future: Pin<Box< dyn Future<Output = Out> >> ) -> Result<crate::JoinHandle<Out>, SpawnError>
	{
		(**self).spawn_handle_local_os( future )
	}
}


impl<T, Out> LocalSpawnHandleOs<Out> for &mut T where T: LocalSpawnHandleOs<Out>, Out: 'static
{
	fn spawn_handle_local_os( &self, future: Pin<Box< dyn Future<Output = Out> >> ) -> Result<crate::JoinHandle<Out>, SpawnError>
	{
		(**self).spawn_handle_local_os( future )
	}
}



#[ cfg(any( feature = "tokio_ct" )) ]
//
impl<Out: 'static> LocalSpawnHandleOs<Out> for crate::TokioLocalHandle
{
	fn spawn_handle_local_os( &self, future: Pin<Box< dyn Future<Output = Out> >> ) -> Result<crate::JoinHandle<Out>, SpawnError>
	{
		self.spawn_handle_local( future )
	}
}



#[ cfg( feature = "bindgen" ) ]
//
impl<Out: 'static> LocalSpawnHandleOs<Out> for crate::Bindgen
{
	fn spawn_handle_local_os( &self, future: Pin<Box< dyn Future<Output = Out> >> ) -> Result<crate::JoinHandle<Out>, SpawnError>
	{
		self.spawn_handle_local( future )
	}
}



#[ cfg( feature = "localpool" ) ]
//
impl<Out: 'static> LocalSpawnHandleOs<Out> for futures_executor::LocalSpawner
{
	fn spawn_handle_local_os( &self, future: Pin<Box< dyn Future<Output = Out> >> ) -> Result<crate::JoinHandle<Out>, SpawnError>
	{
		self.spawn_handle_local( future )
	}
}