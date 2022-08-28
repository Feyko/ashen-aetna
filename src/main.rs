mod context;

use ash::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut context = context::new()?;
    context.choose_device();
    Ok(())
}