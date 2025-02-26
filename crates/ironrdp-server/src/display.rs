pub use ironrdp_acceptor::DesktopSize;
pub use ironrdp_graphics::image_processing::PixelFormat;

/// Display Update
///
/// Contains all types of display updates currently supported by the server implementation
/// and the RDP spec
///
#[derive(Debug, Clone)]
pub enum DisplayUpdate<'life> {
    Bitmap(BitmapUpdate<'life>),
}

#[derive(Debug, Clone, Copy)]
pub enum PixelOrder {
    TopToBottom,
    BottomToTop,
}

/// Bitmap Display Update
///
/// Bitmap updates are encoded using RDP 6.0 compression, fragmented and sent using
/// Fastpath Server Updates
///
#[derive(Debug, Clone)]
pub struct BitmapUpdate<'life> {
    pub top: u32,
    pub left: u32,
    pub width: u32,
    pub height: u32,
    pub format: PixelFormat,
    pub order: PixelOrder,
    pub data: &'life [u8],
}

/// Display Update receiver for an RDP server
///
/// The RDP server will repeatedly call the `get_update` method to receive display updates
/// which will then be encoded and sent to the client
///
/// # Example
///
/// ```
/// use ironrdp_server::{DesktopSize, DisplayUpdate, RdpServerDisplay};
///
/// pub struct DisplayHandler {
///     width: u16,
///     height: u16,
///     receiver: tokio::sync::mpsc::Receiver<DisplayUpdate>,
/// }
///
/// #[async_trait::async_trait]
/// impl RdpServerDisplay for DisplayHandler {
///     async fn size(&mut self) -> DesktopSize {
///         DesktopSize { width: self.width, height: self.height }
///     }
///
///     async fn get_update(&mut self) -> Option<DisplayUpdate> {
///         self.receiver.recv().await
///     }
/// }
/// ```
#[async_trait::async_trait]
pub trait RdpServerDisplay {
    /// This method should return the current size of the display.
    /// Currently, there is no way for the client to negotiate resolution,
    /// so the size returned by this method will be enforced.
    async fn size(&mut self) -> DesktopSize;

    /// # Cancel safety
    ///
    /// This method MUST be cancellation safe because it is used in a `tokio::select!` statement.
    /// If some other branch completes first, it MUST be guaranteed that no data is lost.
    async fn get_update(&mut self) -> Option<DisplayUpdate>;
}
