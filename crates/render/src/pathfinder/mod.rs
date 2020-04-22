
/// The RenderContext2D trait, provides the rendering ctx. It is used for drawing shapes, text, images, and other objects.
pub struct RenderContext2D {
 
}

impl RenderContext2D {
    /// Creates a new render ctx 2d.
    pub fn new(width: f64, height: f64) -> Self {
        RenderContext2D {
          
        }
    }

    pub fn resize(&mut self, width: f64, height: f64) {
       
    }

    /// Registers a new font file.
    pub fn register_font(&mut self, family: &str, font_file: &'static [u8]) {
      
    }

    // Rectangles

    /// Draws a filled rectangle whose starting point is at the coordinates {x, y} with the specified width and height and whose style is determined by the fillStyle attribute.
    pub fn fill_rect(&mut self, x: f64, y: f64, width: f64, height: f64) {
     
    }

    /// Draws a rectangle that is stroked (outlined) according to the current strokeStyle and other ctx settings.
    pub fn stroke_rect(&mut self, x: f64, y: f64, width: f64, height: f64) {
     
    }

    // Text

    /// Draws (fills) a given text at the given (x, y) position.
    pub fn fill_text(&mut self, text: &str, x: f64, y: f64) {
      
    }

    /// Returns a TextMetrics object.
    pub fn measure_text(&mut self, text: &str) -> TextMetrics {
      
    }

    /// Fills the current or given path with the current file style.
    pub fn fill(&mut self) {
    
    }

    /// Strokes {outlines} the current or given path with the current stroke style.
    pub fn stroke(&mut self) {
      
    }

    /// Starts a new path by emptying the list of sub-paths. Call this when you want to create a new path.
    pub fn begin_path(&mut self) {
    
    }

    /// Attempts to add a straight line from the current point to the start of the current sub-path. If the shape has already been closed or has only one point, this function does nothing.
    pub fn close_path(&mut self) {
      
    }

    /// Adds a rectangle to the current path.
    pub fn rect(&mut self, x: f64, y: f64, width: f64, height: f64) {
      
    }

    /// Creates a circular arc centered at (x, y) with a radius of radius. The path starts at startAngle and ends at endAngle.
    pub fn arc(&mut self, x: f64, y: f64, radius: f64, start_angle: f64, end_angle: f64) {
     
    }

    /// Begins a new sub-path at the point specified by the given {x, y} coordinates.

    pub fn move_to(&mut self, x: f64, y: f64) {
      
    }

    /// Adds a straight line to the current sub-path by connecting the sub-path's last point to the specified {x, y} coordinates.
    pub fn line_to(&mut self, x: f64, y: f64) {
     
    }

    /// Adds a quadratic Bézier curve to the current sub-path.
    pub fn quadratic_curve_to(&mut self, cpx: f64, cpy: f64, x: f64, y: f64) {
      
    }

    /// Adds a cubic Bézier curve to the current sub-path. It requires three points: the first two are control points and the third one is the end point. The starting point is the latest point in the current path, which can be changed using MoveTo{} before creating the Bézier curve.
    pub fn bezier_curve_to(&mut self, cp1x: f64, cp1y: f64, cp2x: f64, cp2y: f64, x: f64, y: f64) {
    
    }

    /// Draws a render target.
    pub fn draw_render_target(&mut self, render_target: &RenderTarget, x: f64, y: f64) {
      
    }

    /// Draws the image.
    pub fn draw_image(&mut self, image: &Image, x: f64, y: f64) {
      
    }

    /// Draws the given part of the image.
    pub fn draw_image_with_clip(&mut self, image: &Image, clip: Rectangle, x: f64, y: f64) {
     
    }

    pub fn draw_pipeline(
        &mut self,
        x: f64,
        y: f64,
        width: f64,
        height: f64,
        pipeline: Box<dyn Pipeline>,
    ) {
     
    }

    /// Creates a clipping path from the current sub-paths. Everything drawn after clip() is called appears inside the clipping path only.
    pub fn clip(&mut self) {
     
    }

    // Line styles

    /// Sets the thickness of lines.
    pub fn set_line_width(&mut self, line_width: f64) {
       
    }

    /// Sets the alpha value,
    pub fn set_alpha(&mut self, alpha: f32) {
     
    }

    /// Specific the font family.
    pub fn set_font_family(&mut self, family: impl Into<String>) {
      
    }

    /// Specifies the font size.
    pub fn set_font_size(&mut self, size: f64) {
      
    }

    // Fill and stroke style

    /// Specifies the fill color to use inside shapes.
    pub fn set_fill_style(&mut self, fill_style: Brush) {
      
    }

    /// Specifies the fill stroke to use inside shapes.
    pub fn set_stroke_style(&mut self, stroke_style: Brush) {
      
    }

    // Transformations

    /// Sets the tranformation.
    pub fn set_transform(
        &mut self,
        h_scaling: f64,
        h_skewing: f64,
        v_skewing: f64,
        v_scaling: f64,
        h_moving: f64,
        v_moving: f64,
    ) {
     
    }

    // Canvas states

    /// Saves the entire state of the canvas by pushing the current state onto a stack.
    pub fn save(&mut self) {
        self.saved_config = Some(self.config.clone());
    }

    /// Restores the most recently saved canvas state by popping the top entry in the drawing state stack. If there is no saved state, this method does nothing.
    pub fn restore(&mut self) {
    
    }

    pub fn clear(&mut self, brush: &Brush) {
    
    }

    // pub fn data(&self) -> &[u32] {

    // }

    // pub fn data_mut(&mut self) -> &mut [u32] {

    // }

    // pub fn data_u8_mut(&mut self) -> &mut [u8] {
 
    // }

    pub fn start(&mut self) {}
    pub fn finish(&mut self) {}
}

// --- Conversions ---

impl From<&str> for Image {
    fn from(s: &str) -> Image {
        Image::from_path(s).unwrap()
    }
}

impl From<String> for Image {
    fn from(s: String) -> Image {
        Image::from_path(s).unwrap()
    }
}

// --- Conversions ---


