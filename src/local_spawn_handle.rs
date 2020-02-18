#[ allow(unused_imports) ]
//
use
{
	futures_util :: { task::{ LocalSpawnExt, SpawnError }, future::FutureExt } ,
	std          :: { future::Future, sync::Arc, rc::Rc                      } ,
	crate        :: { import::*, JoinHandle, remote_handle::remote_handle    } ,
};


/// Let's you spawn and get a [JoinHandle] to await the output of a future.
///
/// This trait is not object safe, see [LocalSpawnHandleOs](crate::LocalSpawnHandleOs)
/// for a best effort object safe one.
///
/// ## Performance
///
/// For [tokio] and [async-std](async_std_crate) this is generally faster than [LocalSpawnExt::spawn_local], since
/// it's better aligned with the underlying API and doesn't require extra boxing.
//
#[ cfg_attr( feature = "docs", doc(cfg( feature = "spawn_handle" )) ) ]
//
pub trait LocalSpawnHandle
{
	/// Spawn a future and return a [JoinHandle] that can be awaited for the output of the future.
	//
	fn spawn_handle_local<Fut, Out>( &self, future: Fut ) -> Result<JoinHandle<Out>, SpawnError>

		where Fut: Future<Output = Out> + 'static,
		      Out: 'static
	;
}


impl<T> LocalSpawnHandle for Box<T> where T: LocalSpawnHandle
{
	fn spawn_handle_local<Fut, Out>( &self, future: Fut ) -> Result<JoinHandle<Out>, SpawnError>

		where Fut: Future<Output = Out> + 'static,
		      Out: 'static

	{
		(**self).spawn_handle_local( future )
	}
}


impl<T> LocalSpawnHandle for Arc<T> where T: LocalSpawnHandle
{
	fn spawn_handle_local<Fut, Out>( &self, future: Fut ) -> Result<JoinHandle<Out>, SpawnError>

		where Fut: Future<Output = Out> + 'static,
		      Out: 'static

	{
		(**self).spawn_handle_local( future )
	}
}


impl<T> LocalSpawnHandle for Rc<T> where T: LocalSpawnHandle
{
	fn spawn_handle_local<Fut, Out>( &self, future: Fut ) -> Result<JoinHandle<Out>, SpawnError>

		where Fut: Future<Output = Out> + 'static,
		      Out: 'static

	{
		(**self).spawn_handle_local( future )
	}
}


impl<T> LocalSpawnHandle for &T where T: LocalSpawnHandle
{
	fn spawn_handle_local<Fut, Out>( &self, future: Fut ) -> Result<JoinHandle<Out>, SpawnError>

		where Fut: Future<Output = Out> + 'static,
		      Out: 'static

	{
		(**self).spawn_handle_local( future )
	}
}


impl<T> LocalSpawnHandle for &mut T where T: LocalSpawnHandle
{
	fn spawn_handle_local<Fut, Out>( &self, future: Fut ) -> Result<JoinHandle<Out>, SpawnError>

		where Fut: Future<Output = Out> + 'static,
		      Out: 'static

	{
		(**self).spawn_handle_local( future )
	}
}


#[ cfg( feature = "tracing" ) ]
//
impl<T> LocalSpawnHandle for Instrumented<T> where T: LocalSpawnHandle
{
	fn spawn_handle_local<Fut, Out>( &self, future: Fut ) -> Result<JoinHandle<Out>, SpawnError>

		where Fut: Future<Output = Out> + 'static,
		      Out: 'static

	{
		let fut = future.instrument( self.span().clone() );

		self.inner().spawn_handle_local( fut )
	}
}


#[ cfg( feature = "tracing" ) ]
//
impl<T> LocalSpawnHandle for WithDispatch<T> where T: LocalSpawnHandle
{
	fn spawn_handle_local<Fut, Out>( &self, future: Fut ) -> Result<JoinHandle<Out>, SpawnError>

		where Fut: Future<Output = Out> + 'static,
		      Out: 'static

	{
		let fut = self.with_dispatch(future);

		self.inner().spawn_handle_local( fut )
	}
}



#[ cfg( feature = "tokio_ct" ) ]
//
impl LocalSpawnHandle for crate::TokioCt
{
	fn spawn_handle_local<Fut, Out>( &self, future: Fut ) -> Result<JoinHandle<Out>, SpawnError>

		where Fut: Future<Output = Out> + 'static,
		      Out: 'static

	{
		// tokio's JoinHandle requires Send on the future, so we have to revert to RemoteHandle here.
		// This has some overhead.
		//
		let (fut, handle) = remote_handle( future );
		self.spawn_local(fut)?;

		Ok( JoinHandle{ inner: crate::join_handle::InnerJh::RemoteHandle( Some(handle) ) } )
	}
}



#[ cfg( feature = "bindgen" ) ]
//
impl LocalSpawnHandle for crate::Bindgen
{
	fn spawn_handle_local<Fut, Out>( &self, future: Fut ) -> Result<JoinHandle<Out>, SpawnError>

		where Fut: Future<Output = Out> + 'static,
		      Out: 'static

	{
		let (fut, handle) = remote_handle( future );
		wasm_bindgen_futures::spawn_local(fut);

		Ok( JoinHandle{ inner: crate::join_handle::InnerJh::RemoteHandle( Some(handle) ) } )
	}
}



#[ cfg( feature = "localpool" ) ]
//
impl LocalSpawnHandle for futures_executor::LocalSpawner
{
	fn spawn_handle_local<Fut, Out>( &self, future: Fut ) -> Result<JoinHandle<Out>, SpawnError>

		where Fut: Future<Output = Out> + 'static,
		      Out: 'static

	{
		let (fut, handle) = remote_handle( future );

		self.spawn_local( fut )?;

		Ok( JoinHandle{ inner: crate::join_handle::InnerJh::RemoteHandle( Some(handle) ) } )
	}
}
