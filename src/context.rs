use ash::*;
use ash::extensions::ext;
use ash::vk::{ApplicationInfo, ApplicationInfoBuilder, DebugUtilsMessengerCreateInfoEXTBuilder, PhysicalDevice};

struct Context {
    instance: Instance,
    debug_utils: ext::DebugUtils,
    utils_messenger: vk::DebugUtilsMessengerEXT,
    device: Option<PhysicalDevice>,
}

impl Drop for Context {
    fn drop(&mut self) {
        unsafe {
            self.debug_utils.destroy_debug_utils_messenger(self.utils_messenger, None);
            self.instance.destroy_instance(None)
        };
    }
}

impl Context {
    pub fn choose_device(&mut self) -> Result<PhysicalDevice, Box<dyn std::error::Error>> {
        let device = unsafe { self.instance.enumerate_physical_devices()?[0] };
        self.device = Some(device);
        return Ok(device)
    }
}

pub fn new() -> Result<Context, Box<dyn std::error::Error>> {
    let entry = unsafe {Entry::load()?};
    let app_info = app_info();
    let mut debug_create_info = debug_create_info();
    let instance_create_info = instance_create_info(&mut debug_create_info, &app_info);

    // dbg!(&instance_create_info);
    let instance = unsafe { entry.create_instance(&instance_create_info, None)? };
    let debug_utils = extensions::ext::DebugUtils::new(&entry, &instance);
    let utils_messenger = unsafe { debug_utils.create_debug_utils_messenger(&debug_create_info, None)?};

    return Ok(Context{
        instance: instance,
        debug_utils: debug_utils,
        utils_messenger: utils_messenger,
        device: None,
    })
}

fn app_info<'a>() -> vk::ApplicationInfoBuilder<'a> {
    let enginename = std::ffi::CString::new("QOI-GPU-ENG").unwrap();
    let appname = std::ffi::CString::new("QOI-GPU-GAM").unwrap();
    return vk::ApplicationInfo::builder()
        .application_name(&appname)
        .application_version(vk::make_api_version(0,0,0,0))
        .engine_name(&enginename)
        .engine_version(vk::make_api_version(0,0,0,0))
        .api_version(vk::make_api_version(0,1,3,216));
}

fn debug_create_info<'a>() -> vk::DebugUtilsMessengerCreateInfoEXTBuilder<'a> {
    return vk::DebugUtilsMessengerCreateInfoEXT::builder()
        .message_severity(
            vk::DebugUtilsMessageSeverityFlagsEXT::WARNING |
                vk::DebugUtilsMessageSeverityFlagsEXT::VERBOSE |
                vk::DebugUtilsMessageSeverityFlagsEXT::INFO |
                vk::DebugUtilsMessageSeverityFlagsEXT::ERROR)
        .message_type(
            vk::DebugUtilsMessageTypeFlagsEXT::GENERAL |
                vk::DebugUtilsMessageTypeFlagsEXT::PERFORMANCE |
                vk::DebugUtilsMessageTypeFlagsEXT::VALIDATION)
        .pfn_user_callback(Some(vulkan_debug_utils_callback));
}

fn instance_create_info<'a>(debug_create_info: &mut DebugUtilsMessengerCreateInfoEXTBuilder, app_info: &ApplicationInfoBuilder) -> vk::InstanceCreateInfoBuilder<'a> {
    let layer_names: Vec<std::ffi::CString> = vec![std::ffi::CString::new("VK_LAYER_KHRONOS_validation").unwrap()];
    let layer_name_pointers: Vec<*const i8>  = layer_names.iter().map(|layer_name| layer_name.as_ptr()).collect();
    let extension_name_pointers: Vec<*const i8> = vec![ext::DebugUtils::name().as_ptr()];
    return vk::InstanceCreateInfo::builder()
        .push_next(debug_create_info)
        .application_info(app_info)
        .enabled_layer_names(&layer_name_pointers)
        .enabled_extension_names(&extension_name_pointers);
}

unsafe extern "system" fn vulkan_debug_utils_callback(
    message_severity: vk::DebugUtilsMessageSeverityFlagsEXT,
    message_type: vk::DebugUtilsMessageTypeFlagsEXT,
    p_callback_data: *const vk::DebugUtilsMessengerCallbackDataEXT,
    _p_user_data: *mut std::ffi::c_void,
) -> vk::Bool32 {
    let message = std::ffi::CStr::from_ptr((*p_callback_data).p_message);
    let severity = format!("{:?}", message_severity).to_lowercase();
    let ty = format!("{:?}", message_type).to_lowercase();
    println!("[Debug][{}][{}] {:?}", severity, ty, message);
    vk::FALSE
}