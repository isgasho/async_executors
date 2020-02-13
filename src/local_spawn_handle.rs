use
{
	futures_util :: { task::{ LocalSpawnExt, SpawnError }, future::FutureExt } ,
	std          :: { future::Future, sync::Arc, rc::Rc                      } ,
};


/// Let's you spawn and get a [JoinHandle] to await the output of a future.
///
/// This trait is not object safe, see [LocalSpawnHandleOs] for a best effort object safe one.
//
pub trait LocalSpawnHandle
{
	/// Spawn a future and return a [JoinHandle] that can be awaited for the output of the future.
	//
	fn spawn_handle_local<Fut, Out>( &self, future: Fut ) -> Result<crate::JoinHandle<Out>, SpawnError>

		where Fut: Future<Output = Out> + 'static,
		      Out: 'static
	;
}


impl<T> LocalSpawnHandle for Arc<T> where T: LocalSpawnHandle
{
	fn spawn_handle_local<Fut, Out>( &self, future: Fut ) -> Result<crate::JoinHandle<Out>, SpawnError>

		where Fut: Future<Output = Out> + 'static,
		      Out: 'static

	{
		(**self).spawn_handle_local( future )
	}
}


impl<T> LocalSpawnHandle for Rc<T> where T: LocalSpawnHandle
{
	fn spawn_handle_local<Fut, Out>( &self, future: Fut ) -> Result<crate::JoinHandle<Out>, SpawnError>

		where Fut: Future<Output = Out> + 'static,
		      Out: 'static

	{
		(**self).spawn_handle_local( future )
	}
}


impl<T> LocalSpawnHandle for &T where T: LocalSpawnHandle
{
	fn spawn_handle_local<Fut, Out>( &self, future: Fut ) -> Result<crate::JoinHandle<Out>, SpawnError>

		where Fut: Future<Output = Out> + 'static,
		      Out: 'static

	{
		(**self).spawn_handle_local( future )
	}
}


impl<T> LocalSpawnHandle for &mut T where T: LocalSpawnHandle
{
	fn spawn_handle_local<Fut, Out>( &self, future: Fut ) -> Result<crate::JoinHandle<Out>, SpawnError>

		where Fut: Future<Output = Out> + 'static,
		      Out: 'static

	{
		(**self).spawn_handle_local( future )
	}
}



#[ cfg(any( feature = "tokio_ct" )) ]
//
impl LocalSpawnHandle for crate::TokioLocalHandle
{
	fn spawn_handle_local<Fut, Out>( &self, future: Fut ) -> Result<crate::JoinHandle<Out>, SpawnError>

		where Fut: Future<Output = Out> + 'static,
		      Out: 'static

	{
		// tokio's JoinHandle requires Send on the future, so we have to revert to RemoteHandle here.
		// This has some overhead.
		//
		let (fut, handle) = future.remote_handle();
		self.spawn_local(fut)?;

		Ok( crate::JoinHandle::RemoteHandle( Some(handle) ) )
	}
}



#[ cfg( feature = "bindgen" ) ]
//
impl LocalSpawnHandle for crate::Bindgen
{
	fn spawn_handle_local<Fut, Out>( &self, future: Fut ) -> Result<crate::JoinHandle<Out>, SpawnError>

		where Fut: Future<Output = Out> + 'static,
		      Out: 'static

	{
		let (fut, handle) = future.remote_handle();
		wasm_bindgen_futures::spawn_local(fut);

		Ok( crate::JoinHandle::RemoteHandle( Some(handle) ) )
	}
}



#[ cfg( feature = "localpool" ) ]
//
impl LocalSpawnHandle for futures_executor::LocalSpawner
{
	fn spawn_handle_local<Fut, Out>( &self, future: Fut ) -> Result<crate::JoinHandle<Out>, SpawnError>

		where Fut: Future<Output = Out> + 'static,
		      Out: 'static

	{
		let handle = self.spawn_local_with_handle( future )?;

		Ok( crate::JoinHandle::RemoteHandle( Some(handle) ) )
	}
}