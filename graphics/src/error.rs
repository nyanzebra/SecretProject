#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("wgpu error: {0}")]
    Wgpu(#[from] wgpu::Error),

    #[error("wgpu error: {0}")]
    WgpuBufferAsync(#[from] wgpu::BufferAsyncError),

    #[error("wgpu error: {0}")]
    WgpuCreateSurface(#[from] wgpu::CreateSurfaceError),

    #[error("wgpu error: {0}")]
    WgpuRequestDevice(#[from] wgpu::RequestDeviceError),

    #[error("wgpu error: {0}")]
    WgpuSurface(#[from] wgpu::SurfaceError),

    #[error("winit error: {0}")]
    WinitEventLoop(#[from] winit::error::EventLoopError),

    #[error("winit error: {0}")]
    WinitExternal(#[from] winit::error::ExternalError),

    #[error("winit error: {0}")]
    WinitNotSupported(#[from] winit::error::NotSupportedError),

    #[error("winit error: {0}")]
    WinitOS(#[from] winit::error::OsError),
}
