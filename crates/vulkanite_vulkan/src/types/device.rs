use ash::vk;
use std::ffi::CString;

#[derive(Debug, Copy, Clone)]
pub struct Features {
    pub robust_buffer_access: bool,
    pub full_draw_index_uint32: bool,
    pub image_cube_array: bool,
    pub independent_blend: bool,
    pub geometry_shader: bool,
    pub tessellation_shader: bool,
    pub sample_rate_shading: bool,
    pub dual_src_blend: bool,
    pub logic_op: bool,
    pub multi_draw_indirect: bool,
    pub draw_indirect_first_instance: bool,
    pub depth_clamp: bool,
    pub depth_bias_clamp: bool,
    pub fill_mode_non_solid: bool,
    pub depth_bounds: bool,
    pub wide_lines: bool,
    pub large_points: bool,
    pub alpha_to_one: bool,
    pub multi_viewport: bool,
    pub sampler_anisotropy: bool,
    pub texture_compression_etc2: bool,
    pub texture_compression_astcldr: bool,
    pub texture_compression_bc: bool,
    pub occlusion_query_precise: bool,
    pub pipeline_statistics_query: bool,
    pub vertex_pipeline_stores_and_atomics: bool,
    pub fragment_stores_and_atomics: bool,
    pub shader_tessellation_and_geometry_point_size: bool,
    pub shader_image_gather_extended: bool,
    pub shader_storage_image_extended_formats: bool,
    pub shader_storage_image_multisample: bool,
    pub shader_storage_image_read_without_format: bool,
    pub shader_storage_image_write_without_format: bool,
    pub shader_uniform_buffer_array_dynamic_indexing: bool,
    pub shader_sampled_image_array_dynamic_indexing: bool,
    pub shader_storage_buffer_array_dynamic_indexing: bool,
    pub shader_storage_image_array_dynamic_indexing: bool,
    pub shader_clip_distance: bool,
    pub shader_cull_distance: bool,
    pub shader_float64: bool,
    pub shader_int64: bool,
    pub shader_int16: bool,
    pub shader_resource_residency: bool,
    pub shader_resource_min_lod: bool,
    pub sparse_binding: bool,
    pub sparse_residency_buffer: bool,
    pub sparse_residency_image2d: bool,
    pub sparse_residency_image3d: bool,
    pub sparse_residency2samples: bool,
    pub sparse_residency4samples: bool,
    pub sparse_residency8samples: bool,
    pub sparse_residency16samples: bool,
    pub sparse_residency_aliased: bool,
    pub variable_multisample_rate: bool,
    pub inherited_queries: bool,
}

impl Features {
    pub const fn none() -> Self {
        Self {
            robust_buffer_access: false,
            full_draw_index_uint32: false,
            image_cube_array: false,
            independent_blend: false,
            geometry_shader: false,
            tessellation_shader: false,
            sample_rate_shading: false,
            dual_src_blend: false,
            logic_op: false,
            multi_draw_indirect: false,
            draw_indirect_first_instance: false,
            depth_clamp: false,
            depth_bias_clamp: false,
            fill_mode_non_solid: false,
            depth_bounds: false,
            wide_lines: false,
            large_points: false,
            alpha_to_one: false,
            multi_viewport: false,
            sampler_anisotropy: false,
            texture_compression_etc2: false,
            texture_compression_astcldr: false,
            texture_compression_bc: false,
            occlusion_query_precise: false,
            pipeline_statistics_query: false,
            vertex_pipeline_stores_and_atomics: false,
            fragment_stores_and_atomics: false,
            shader_tessellation_and_geometry_point_size: false,
            shader_image_gather_extended: false,
            shader_storage_image_extended_formats: false,
            shader_storage_image_multisample: false,
            shader_storage_image_read_without_format: false,
            shader_storage_image_write_without_format: false,
            shader_uniform_buffer_array_dynamic_indexing: false,
            shader_sampled_image_array_dynamic_indexing: false,
            shader_storage_buffer_array_dynamic_indexing: false,
            shader_storage_image_array_dynamic_indexing: false,
            shader_clip_distance: false,
            shader_cull_distance: false,
            shader_float64: false,
            shader_int64: false,
            shader_int16: false,
            shader_resource_residency: false,
            shader_resource_min_lod: false,
            sparse_binding: false,
            sparse_residency_buffer: false,
            sparse_residency_image2d: false,
            sparse_residency_image3d: false,
            sparse_residency2samples: false,
            sparse_residency4samples: false,
            sparse_residency8samples: false,
            sparse_residency16samples: false,
            sparse_residency_aliased: false,
            variable_multisample_rate: false,
            inherited_queries: false,
        }
    }
}

impl From<Features> for vk::PhysicalDeviceFeatures {
    fn from(features: Features) -> Self {
        Self {
            robust_buffer_access: if features.robust_buffer_access { 1 } else { 0 },
            full_draw_index_uint32: if features.full_draw_index_uint32 {
                1
            } else {
                0
            },
            image_cube_array: if features.image_cube_array { 1 } else { 0 },
            independent_blend: if features.independent_blend { 1 } else { 0 },
            geometry_shader: if features.geometry_shader { 1 } else { 0 },
            tessellation_shader: if features.tessellation_shader { 1 } else { 0 },
            sample_rate_shading: if features.sample_rate_shading { 1 } else { 0 },
            dual_src_blend: if features.dual_src_blend { 1 } else { 0 },
            logic_op: if features.logic_op { 1 } else { 0 },
            multi_draw_indirect: if features.multi_draw_indirect { 1 } else { 0 },
            draw_indirect_first_instance: if features.draw_indirect_first_instance {
                1
            } else {
                0
            },
            depth_clamp: if features.depth_clamp { 1 } else { 0 },
            depth_bias_clamp: if features.depth_bias_clamp { 1 } else { 0 },
            fill_mode_non_solid: if features.fill_mode_non_solid { 1 } else { 0 },
            depth_bounds: if features.depth_bounds { 1 } else { 0 },
            wide_lines: if features.wide_lines { 1 } else { 0 },
            large_points: if features.large_points { 1 } else { 0 },
            alpha_to_one: if features.alpha_to_one { 1 } else { 0 },
            multi_viewport: if features.multi_viewport { 1 } else { 0 },
            sampler_anisotropy: if features.sampler_anisotropy { 1 } else { 0 },
            texture_compression_etc2: if features.texture_compression_etc2 {
                1
            } else {
                0
            },
            texture_compression_astc_ldr: if features.texture_compression_astcldr {
                1
            } else {
                0
            },
            texture_compression_bc: if features.texture_compression_bc {
                1
            } else {
                0
            },
            occlusion_query_precise: if features.occlusion_query_precise {
                1
            } else {
                0
            },
            pipeline_statistics_query: if features.pipeline_statistics_query {
                1
            } else {
                0
            },
            vertex_pipeline_stores_and_atomics: if features.vertex_pipeline_stores_and_atomics {
                1
            } else {
                0
            },
            fragment_stores_and_atomics: if features.fragment_stores_and_atomics {
                1
            } else {
                0
            },
            shader_tessellation_and_geometry_point_size: if features
                .shader_tessellation_and_geometry_point_size
            {
                1
            } else {
                0
            },
            shader_image_gather_extended: if features.shader_image_gather_extended {
                1
            } else {
                0
            },
            shader_storage_image_extended_formats: if features.shader_storage_image_extended_formats
            {
                1
            } else {
                0
            },
            shader_storage_image_multisample: if features.shader_storage_image_multisample {
                1
            } else {
                0
            },
            shader_storage_image_read_without_format: if features
                .shader_storage_image_read_without_format
            {
                1
            } else {
                0
            },
            shader_storage_image_write_without_format: if features
                .shader_storage_image_write_without_format
            {
                1
            } else {
                0
            },
            shader_uniform_buffer_array_dynamic_indexing: if features
                .shader_uniform_buffer_array_dynamic_indexing
            {
                1
            } else {
                0
            },
            shader_sampled_image_array_dynamic_indexing: if features
                .shader_sampled_image_array_dynamic_indexing
            {
                1
            } else {
                0
            },
            shader_storage_buffer_array_dynamic_indexing: if features
                .shader_storage_buffer_array_dynamic_indexing
            {
                1
            } else {
                0
            },
            shader_storage_image_array_dynamic_indexing: if features
                .shader_storage_image_array_dynamic_indexing
            {
                1
            } else {
                0
            },
            shader_clip_distance: if features.shader_clip_distance { 1 } else { 0 },
            shader_cull_distance: if features.shader_cull_distance { 1 } else { 0 },
            shader_float64: if features.shader_float64 { 1 } else { 0 },
            shader_int64: if features.shader_int64 { 1 } else { 0 },
            shader_int16: if features.shader_int16 { 1 } else { 0 },
            shader_resource_residency: if features.shader_resource_residency {
                1
            } else {
                0
            },
            shader_resource_min_lod: if features.shader_resource_min_lod {
                1
            } else {
                0
            },
            sparse_binding: if features.sparse_binding { 1 } else { 0 },
            sparse_residency_buffer: if features.sparse_residency_buffer {
                1
            } else {
                0
            },
            sparse_residency_image2_d: if features.sparse_residency_image2d {
                1
            } else {
                0
            },
            sparse_residency_image3_d: if features.sparse_residency_image3d {
                1
            } else {
                0
            },
            sparse_residency2_samples: if features.sparse_residency2samples {
                1
            } else {
                0
            },
            sparse_residency4_samples: if features.sparse_residency4samples {
                1
            } else {
                0
            },
            sparse_residency8_samples: if features.sparse_residency8samples {
                1
            } else {
                0
            },
            sparse_residency16_samples: if features.sparse_residency16samples {
                1
            } else {
                0
            },
            sparse_residency_aliased: if features.sparse_residency_aliased {
                1
            } else {
                0
            },
            variable_multisample_rate: if features.variable_multisample_rate {
                1
            } else {
                0
            },
            inherited_queries: if features.inherited_queries { 1 } else { 0 },
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Extensions {
    pub handles: bool,
    pub vk_amd_buffer_marker: bool,
    pub vk_amd_device_coherent_memory: bool,
    pub vk_amd_display_native_hdr: bool,
    pub vk_amd_draw_indirect_count: bool,
    pub vk_amd_gcn_shader: bool,
    pub vk_amd_gpu_shader_half_float: bool,
    pub vk_amd_gpu_shader_int16: bool,
    pub vk_amd_memory_overallocation_behavior: bool,
    pub vk_amd_mixed_attachment_samples: bool,
    pub vk_amd_negative_viewport_height: bool,
    pub vk_amd_pipeline_compiler_control: bool,
    pub vk_amd_rasterization_order: bool,
    pub vk_amd_shader_ballot: bool,
    pub vk_amd_shader_core_properties: bool,
    pub vk_amd_shader_core_properties2: bool,
    pub vk_amd_shader_explicit_vertex_parameter: bool,
    pub vk_amd_shader_fragment_mask: bool,
    pub vk_amd_shader_image_load_store_lod: bool,
    pub vk_amd_shader_info: bool,
    pub vk_amd_shader_trinary_minmax: bool,
    pub vk_amd_texture_gather_bias_lod: bool,
    pub vk_android_external_memory_android_hardware_buffer: bool,
    pub vk_ext_acquire_xlib_display: bool,
    pub vk_ext_astc_decode_mode: bool,
    pub vk_ext_blend_operation_advanced: bool,
    pub vk_ext_buffer_device_address: bool,
    pub vk_ext_calibrated_timestamps: bool,
    pub vk_ext_conditional_rendering: bool,
    pub vk_ext_conservative_rasterization: bool,
    pub vk_ext_custom_border_color: bool,
    pub vk_ext_debug_marker: bool,
    pub vk_ext_debug_report: bool,
    pub vk_ext_debug_utils: bool,
    pub vk_ext_depth_clip_enable: bool,
    pub vk_ext_depth_range_unrestricted: bool,
    pub vk_ext_descriptor_indexing: bool,
    pub vk_ext_direct_mode_display: bool,
    pub vk_ext_discard_rectangles: bool,
    pub vk_ext_display_control: bool,
    pub vk_ext_display_surface_counter: bool,
    pub vk_ext_external_memory_dma_buf: bool,
    pub vk_ext_external_memory_host: bool,
    pub vk_ext_filter_cubic: bool,
    pub vk_ext_fragment_density_map: bool,
    pub vk_ext_fragment_shader_interlock: bool,
    pub vk_ext_full_screen_exclusive: bool,
    pub vk_ext_global_priority: bool,
    pub vk_ext_hdr_metadata: bool,
    pub vk_ext_headless_surface: bool,
    pub vk_ext_host_query_reset: bool,
    pub vk_ext_image_drm_format_modifier: bool,
    pub vk_ext_index_type_uint8: bool,
    pub vk_ext_inline_uniform_block: bool,
    pub vk_ext_line_rasterization: bool,
    pub vk_ext_memory_budget: bool,
    pub vk_ext_memory_priority: bool,
    pub vk_ext_metal_surface: bool,
    pub vk_ext_pci_bus_info: bool,
    pub vk_ext_pipeline_creation_cache_control: bool,
    pub vk_ext_pipeline_creation_feedback: bool,
    pub vk_ext_post_depth_coverage: bool,
    pub vk_ext_private_data: bool,
    pub vk_ext_queue_family_foreign: bool,
    pub vk_ext_robustness2: bool,
    pub vk_ext_sample_locations: bool,
    pub vk_ext_sampler_filter_minmax: bool,
    pub vk_ext_scalar_block_layout: bool,
    pub vk_ext_separate_stencil_usage: bool,
    pub vk_ext_shader_demote_to_helper_invocation: bool,
    pub vk_ext_shader_stencil_export: bool,
    pub vk_ext_shader_subgroup_ballot: bool,
    pub vk_ext_shader_subgroup_vote: bool,
    pub vk_ext_shader_viewport_index_layer: bool,
    pub vk_ext_subgroup_size_control: bool,
    pub vk_ext_swapchain_colorspace: bool,
    pub vk_ext_texel_buffer_alignment: bool,
    pub vk_ext_texture_compression_astc_hdr: bool,
    pub vk_ext_tooling_info: bool,
    pub vk_ext_transform_feedback: bool,
    pub vk_ext_validation_cache: bool,
    pub vk_ext_validation_features: bool,
    pub vk_ext_validation_flags: bool,
    pub vk_ext_vertex_attribute_divisor: bool,
    pub vk_ext_ycbcr_image_arrays: bool,
    pub vk_fuchsia_imagepipe_surface: bool,
    pub vk_ggp_frame_token: bool,
    pub vk_ggp_stream_descriptor_surface: bool,
    pub vk_google_decorate_string: bool,
    pub vk_google_display_timing: bool,
    pub vk_google_hlsl_functionality1: bool,
    pub vk_google_user_type: bool,
    pub vk_img_filter_cubic: bool,
    pub vk_img_format_pvrtc: bool,
    pub vk_intel_performance_query: bool,
    pub vk_intel_shader_integer_functions2: bool,
    pub vk_khr_16bit_storage: bool,
    pub vk_khr_8bit_storage: bool,
    pub vk_khr_acceleration_structure: bool,
    pub vk_khr_android_surface: bool,
    pub vk_khr_bind_memory2: bool,
    pub vk_khr_buffer_device_address: bool,
    pub vk_khr_create_renderpass2: bool,
    pub vk_khr_dedicated_allocation: bool,
    pub vk_khr_deferred_host_operations: bool,
    pub vk_khr_depth_stencil_resolve: bool,
    pub vk_khr_descriptor_update_template: bool,
    pub vk_khr_device_group: bool,
    pub vk_khr_device_group_creation: bool,
    pub vk_khr_display: bool,
    pub vk_khr_display_swapchain: bool,
    pub vk_khr_draw_indirect_count: bool,
    pub vk_khr_driver_properties: bool,
    pub vk_khr_dynamic_rendering: bool,
    pub vk_khr_external_fence: bool,
    pub vk_khr_external_fence_capabilities: bool,
    pub vk_khr_external_fence_fd: bool,
    pub vk_khr_external_fence_win32: bool,
    pub vk_khr_external_memory: bool,
    pub vk_khr_external_memory_capabilities: bool,
    pub vk_khr_external_memory_fd: bool,
    pub vk_khr_external_memory_win32: bool,
    pub vk_khr_external_semaphore: bool,
    pub vk_khr_external_semaphore_capabilities: bool,
    pub vk_khr_external_semaphore_fd: bool,
    pub vk_khr_external_semaphore_win32: bool,
    pub vk_khr_get_display_properties2: bool,
    pub vk_khr_get_memory_requirements2: bool,
    pub vk_khr_get_physical_device_properties2: bool,
    pub vk_khr_get_surface_capabilities2: bool,
    pub vk_khr_image_format_list: bool,
    pub vk_khr_imageless_framebuffer: bool,
    pub vk_khr_incremental_present: bool,
    pub vk_khr_maintenance1: bool,
    pub vk_khr_maintenance2: bool,
    pub vk_khr_maintenance3: bool,
    pub vk_khr_multiview: bool,
    pub vk_khr_performance_query: bool,
    pub vk_khr_pipeline_executable_properties: bool,
    pub vk_khr_pipeline_library: bool,
    pub vk_khr_push_descriptor: bool,
    pub vk_khr_ray_tracing_pipeline: bool,
    pub vk_khr_ray_query: bool,
    pub vk_khr_relaxed_block_layout: bool,
    pub vk_khr_sampler_mirror_clamp_to_edge: bool,
    pub vk_khr_sampler_ycbcr_conversion: bool,
    pub vk_khr_separate_depth_stencil_layouts: bool,
    pub vk_khr_shader_atomic_int64: bool,
    pub vk_khr_shader_clock: bool,
    pub vk_khr_shader_draw_parameters: bool,
    pub vk_khr_shader_float16_int8: bool,
    pub vk_khr_shader_float_controls: bool,
    pub vk_khr_shader_non_semantic_info: bool,
    pub vk_khr_shader_subgroup_extended_types: bool,
    pub vk_khr_shared_presentable_image: bool,
    pub vk_khr_spirv_1_4: bool,
    pub vk_khr_storage_buffer_storage_class: bool,
    pub vk_khr_surface: bool,
    pub vk_khr_surface_protected_capabilities: bool,
    pub vk_khr_swapchain: bool,
    pub vk_khr_swapchain_mutable_format: bool,
    pub vk_khr_timeline_semaphore: bool,
    pub vk_khr_uniform_buffer_standard_layout: bool,
    pub vk_khr_variable_pointers: bool,
    pub vk_khr_vulkan_memory_model: bool,
    pub vk_khr_wayland_surface: bool,
    pub vk_khr_win32_keyed_mutex: bool,
    pub vk_khr_win32_surface: bool,
    pub vk_khr_xcb_surface: bool,
    pub vk_khr_xlib_surface: bool,
    pub vk_mvk_ios_surface: bool,
    pub vk_mvk_macos_surface: bool,
    pub vk_nn_vi_surface: bool,
    pub vk_nvx_image_view_handle: bool,
    pub vk_nvx_multiview_per_view_attributes: bool,
    pub vk_nv_clip_space_w_scaling: bool,
    pub vk_nv_compute_shader_derivatives: bool,
    pub vk_nv_cooperative_matrix: bool,
    pub vk_nv_corner_sampled_image: bool,
    pub vk_nv_coverage_reduction_mode: bool,
    pub vk_nv_dedicated_allocation: bool,
    pub vk_nv_dedicated_allocation_image_aliasing: bool,
    pub vk_nv_device_diagnostic_checkpoints: bool,
    pub vk_nv_device_diagnostics_config: bool,
    pub vk_nv_device_generated_commands: bool,
    pub vk_nv_external_memory: bool,
    pub vk_nv_external_memory_capabilities: bool,
    pub vk_nv_external_memory_win32: bool,
    pub vk_nv_fill_rectangle: bool,
    pub vk_nv_fragment_coverage_to_color: bool,
    pub vk_nv_fragment_shader_barycentric: bool,
    pub vk_nv_framebuffer_mixed_samples: bool,
    pub vk_nv_geometry_shader_passthrough: bool,
    pub vk_nv_glsl_shader: bool,
    pub vk_nv_mesh_shader: bool,
    pub vk_nv_ray_tracing: bool,
    pub vk_nv_representative_fragment_test: bool,
    pub vk_nv_sample_mask_override_coverage: bool,
    pub vk_nv_scissor_exclusive: bool,
    pub vk_nv_shader_image_footprint: bool,
    pub vk_nv_shader_sm_builtins: bool,
    pub vk_nv_shader_subgroup_partitioned: bool,
    pub vk_nv_shading_rate_image: bool,
    pub vk_nv_viewport_array2: bool,
    pub vk_nv_viewport_swizzle: bool,
    pub vk_nv_win32_keyed_mutex: bool,
    pub vk_qcom_render_pass_shader_resolve: bool,
    pub vk_qcom_render_pass_store_ops: bool,
    pub vk_qcom_render_pass_transform: bool,
    pub wsitypes: bool,
}

impl Extensions {
    pub fn none() -> Self {
        Self {
            handles: false,
            vk_amd_buffer_marker: false,
            vk_amd_device_coherent_memory: false,
            vk_amd_display_native_hdr: false,
            vk_amd_draw_indirect_count: false,
            vk_amd_gcn_shader: false,
            vk_amd_gpu_shader_half_float: false,
            vk_amd_gpu_shader_int16: false,
            vk_amd_memory_overallocation_behavior: false,
            vk_amd_mixed_attachment_samples: false,
            vk_amd_negative_viewport_height: false,
            vk_amd_pipeline_compiler_control: false,
            vk_amd_rasterization_order: false,
            vk_amd_shader_ballot: false,
            vk_amd_shader_core_properties: false,
            vk_amd_shader_core_properties2: false,
            vk_amd_shader_explicit_vertex_parameter: false,
            vk_amd_shader_fragment_mask: false,
            vk_amd_shader_image_load_store_lod: false,
            vk_amd_shader_info: false,
            vk_amd_shader_trinary_minmax: false,
            vk_amd_texture_gather_bias_lod: false,
            vk_android_external_memory_android_hardware_buffer: false,
            vk_ext_acquire_xlib_display: false,
            vk_ext_astc_decode_mode: false,
            vk_ext_blend_operation_advanced: false,
            vk_ext_buffer_device_address: false,
            vk_ext_calibrated_timestamps: false,
            vk_ext_conditional_rendering: false,
            vk_ext_conservative_rasterization: false,
            vk_ext_custom_border_color: false,
            vk_ext_debug_marker: false,
            vk_ext_debug_report: false,
            vk_ext_debug_utils: false,
            vk_ext_depth_clip_enable: false,
            vk_ext_depth_range_unrestricted: false,
            vk_ext_descriptor_indexing: false,
            vk_ext_direct_mode_display: false,
            vk_ext_discard_rectangles: false,
            vk_ext_display_control: false,
            vk_ext_display_surface_counter: false,
            vk_ext_external_memory_dma_buf: false,
            vk_ext_external_memory_host: false,
            vk_ext_filter_cubic: false,
            vk_ext_fragment_density_map: false,
            vk_ext_fragment_shader_interlock: false,
            vk_ext_full_screen_exclusive: false,
            vk_ext_global_priority: false,
            vk_ext_hdr_metadata: false,
            vk_ext_headless_surface: false,
            vk_ext_host_query_reset: false,
            vk_ext_image_drm_format_modifier: false,
            vk_ext_index_type_uint8: false,
            vk_ext_inline_uniform_block: false,
            vk_ext_line_rasterization: false,
            vk_ext_memory_budget: false,
            vk_ext_memory_priority: false,
            vk_ext_metal_surface: false,
            vk_ext_pci_bus_info: false,
            vk_ext_pipeline_creation_cache_control: false,
            vk_ext_pipeline_creation_feedback: false,
            vk_ext_post_depth_coverage: false,
            vk_ext_private_data: false,
            vk_ext_queue_family_foreign: false,
            vk_ext_robustness2: false,
            vk_ext_sample_locations: false,
            vk_ext_sampler_filter_minmax: false,
            vk_ext_scalar_block_layout: false,
            vk_ext_separate_stencil_usage: false,
            vk_ext_shader_demote_to_helper_invocation: false,
            vk_ext_shader_stencil_export: false,
            vk_ext_shader_subgroup_ballot: false,
            vk_ext_shader_subgroup_vote: false,
            vk_ext_shader_viewport_index_layer: false,
            vk_ext_subgroup_size_control: false,
            vk_ext_swapchain_colorspace: false,
            vk_ext_texel_buffer_alignment: false,
            vk_ext_texture_compression_astc_hdr: false,
            vk_ext_tooling_info: false,
            vk_ext_transform_feedback: false,
            vk_ext_validation_cache: false,
            vk_ext_validation_features: false,
            vk_ext_validation_flags: false,
            vk_ext_vertex_attribute_divisor: false,
            vk_ext_ycbcr_image_arrays: false,
            vk_fuchsia_imagepipe_surface: false,
            vk_ggp_frame_token: false,
            vk_ggp_stream_descriptor_surface: false,
            vk_google_decorate_string: false,
            vk_google_display_timing: false,
            vk_google_hlsl_functionality1: false,
            vk_google_user_type: false,
            vk_img_filter_cubic: false,
            vk_img_format_pvrtc: false,
            vk_intel_performance_query: false,
            vk_intel_shader_integer_functions2: false,
            vk_khr_16bit_storage: false,
            vk_khr_8bit_storage: false,
            vk_khr_acceleration_structure: false,
            vk_khr_android_surface: false,
            vk_khr_bind_memory2: false,
            vk_khr_buffer_device_address: false,
            vk_khr_create_renderpass2: false,
            vk_khr_dedicated_allocation: false,
            vk_khr_deferred_host_operations: false,
            vk_khr_depth_stencil_resolve: false,
            vk_khr_descriptor_update_template: false,
            vk_khr_device_group: false,
            vk_khr_device_group_creation: false,
            vk_khr_display: false,
            vk_khr_display_swapchain: false,
            vk_khr_draw_indirect_count: false,
            vk_khr_driver_properties: false,
            vk_khr_dynamic_rendering: false,
            vk_khr_external_fence: false,
            vk_khr_external_fence_capabilities: false,
            vk_khr_external_fence_fd: false,
            vk_khr_external_fence_win32: false,
            vk_khr_external_memory: false,
            vk_khr_external_memory_capabilities: false,
            vk_khr_external_memory_fd: false,
            vk_khr_external_memory_win32: false,
            vk_khr_external_semaphore: false,
            vk_khr_external_semaphore_capabilities: false,
            vk_khr_external_semaphore_fd: false,
            vk_khr_external_semaphore_win32: false,
            vk_khr_get_display_properties2: false,
            vk_khr_get_memory_requirements2: false,
            vk_khr_get_physical_device_properties2: false,
            vk_khr_get_surface_capabilities2: false,
            vk_khr_image_format_list: false,
            vk_khr_imageless_framebuffer: false,
            vk_khr_incremental_present: false,
            vk_khr_maintenance1: false,
            vk_khr_maintenance2: false,
            vk_khr_maintenance3: false,
            vk_khr_multiview: false,
            vk_khr_performance_query: false,
            vk_khr_pipeline_executable_properties: false,
            vk_khr_pipeline_library: false,
            vk_khr_push_descriptor: false,
            vk_khr_ray_tracing_pipeline: false,
            vk_khr_ray_query: false,
            vk_khr_relaxed_block_layout: false,
            vk_khr_sampler_mirror_clamp_to_edge: false,
            vk_khr_sampler_ycbcr_conversion: false,
            vk_khr_separate_depth_stencil_layouts: false,
            vk_khr_shader_atomic_int64: false,
            vk_khr_shader_clock: false,
            vk_khr_shader_draw_parameters: false,
            vk_khr_shader_float16_int8: false,
            vk_khr_shader_float_controls: false,
            vk_khr_shader_non_semantic_info: false,
            vk_khr_shader_subgroup_extended_types: false,
            vk_khr_shared_presentable_image: false,
            vk_khr_spirv_1_4: false,
            vk_khr_storage_buffer_storage_class: false,
            vk_khr_surface: false,
            vk_khr_surface_protected_capabilities: false,
            vk_khr_swapchain: false,
            vk_khr_swapchain_mutable_format: false,
            vk_khr_timeline_semaphore: false,
            vk_khr_uniform_buffer_standard_layout: false,
            vk_khr_variable_pointers: false,
            vk_khr_vulkan_memory_model: false,
            vk_khr_wayland_surface: false,
            vk_khr_win32_keyed_mutex: false,
            vk_khr_win32_surface: false,
            vk_khr_xcb_surface: false,
            vk_khr_xlib_surface: false,
            vk_mvk_ios_surface: false,
            vk_mvk_macos_surface: false,
            vk_nn_vi_surface: false,
            vk_nvx_image_view_handle: false,
            vk_nvx_multiview_per_view_attributes: false,
            vk_nv_clip_space_w_scaling: false,
            vk_nv_compute_shader_derivatives: false,
            vk_nv_cooperative_matrix: false,
            vk_nv_corner_sampled_image: false,
            vk_nv_coverage_reduction_mode: false,
            vk_nv_dedicated_allocation: false,
            vk_nv_dedicated_allocation_image_aliasing: false,
            vk_nv_device_diagnostic_checkpoints: false,
            vk_nv_device_diagnostics_config: false,
            vk_nv_device_generated_commands: false,
            vk_nv_external_memory: false,
            vk_nv_external_memory_capabilities: false,
            vk_nv_external_memory_win32: false,
            vk_nv_fill_rectangle: false,
            vk_nv_fragment_coverage_to_color: false,
            vk_nv_fragment_shader_barycentric: false,
            vk_nv_framebuffer_mixed_samples: false,
            vk_nv_geometry_shader_passthrough: false,
            vk_nv_glsl_shader: false,
            vk_nv_mesh_shader: false,
            vk_nv_ray_tracing: false,
            vk_nv_representative_fragment_test: false,
            vk_nv_sample_mask_override_coverage: false,
            vk_nv_scissor_exclusive: false,
            vk_nv_shader_image_footprint: false,
            vk_nv_shader_sm_builtins: false,
            vk_nv_shader_subgroup_partitioned: false,
            vk_nv_shading_rate_image: false,
            vk_nv_viewport_array2: false,
            vk_nv_viewport_swizzle: false,
            vk_nv_win32_keyed_mutex: false,
            vk_qcom_render_pass_shader_resolve: false,
            vk_qcom_render_pass_store_ops: false,
            vk_qcom_render_pass_transform: false,
            wsitypes: false,
        }
    }
}

impl From<Extensions> for Vec<CString> {
    fn from(device_extensions: Extensions) -> Self {
        let mut extensions = vec![];
        if device_extensions.handles {
            extensions.push(CString::new(&""[..]).unwrap());
        }

        if device_extensions.vk_amd_buffer_marker {
            extensions.push(CString::new(&"VK_AMD_buffer_marker"[..]).unwrap());
        }

        if device_extensions.vk_amd_device_coherent_memory {
            extensions.push(CString::new(&"VK_AMD_device_coherent_memory"[..]).unwrap());
        }

        if device_extensions.vk_amd_display_native_hdr {
            extensions.push(CString::new(&"VK_AMD_display_native_hdr"[..]).unwrap());
        }

        if device_extensions.vk_amd_draw_indirect_count {
            extensions.push(CString::new(&"VK_AMD_draw_indirect_count"[..]).unwrap());
        }

        if device_extensions.vk_amd_gcn_shader {
            extensions.push(CString::new(&"VK_AMD_gcn_shader"[..]).unwrap());
        }

        if device_extensions.vk_amd_gpu_shader_half_float {
            extensions.push(CString::new(&"VK_AMD_gpu_shader_half_float"[..]).unwrap());
        }

        if device_extensions.vk_amd_gpu_shader_int16 {
            extensions.push(CString::new(&"VK_AMD_gpu_shader_int16"[..]).unwrap());
        }

        if device_extensions.vk_amd_memory_overallocation_behavior {
            extensions.push(CString::new(&"VK_AMD_memory_overallocation_behavior"[..]).unwrap());
        }

        if device_extensions.vk_amd_mixed_attachment_samples {
            extensions.push(CString::new(&"VK_AMD_mixed_attachment_samples"[..]).unwrap());
        }

        if device_extensions.vk_amd_negative_viewport_height {
            extensions.push(CString::new(&"VK_AMD_negative_viewport_height"[..]).unwrap());
        }

        if device_extensions.vk_amd_pipeline_compiler_control {
            extensions.push(CString::new(&"VK_AMD_pipeline_compiler_control"[..]).unwrap());
        }

        if device_extensions.vk_amd_rasterization_order {
            extensions.push(CString::new(&"VK_AMD_rasterization_order"[..]).unwrap());
        }

        if device_extensions.vk_amd_shader_ballot {
            extensions.push(CString::new(&"VK_AMD_shader_ballot"[..]).unwrap());
        }

        if device_extensions.vk_amd_shader_core_properties {
            extensions.push(CString::new(&"VK_AMD_shader_core_properties"[..]).unwrap());
        }

        if device_extensions.vk_amd_shader_core_properties2 {
            extensions.push(CString::new(&"VK_AMD_shader_core_properties2"[..]).unwrap());
        }

        if device_extensions.vk_amd_shader_explicit_vertex_parameter {
            extensions.push(CString::new(&"VK_AMD_shader_explicit_vertex_parameter"[..]).unwrap());
        }

        if device_extensions.vk_amd_shader_fragment_mask {
            extensions.push(CString::new(&"VK_AMD_shader_fragment_mask"[..]).unwrap());
        }

        if device_extensions.vk_amd_shader_image_load_store_lod {
            extensions.push(CString::new(&"VK_AMD_shader_image_load_store_lod"[..]).unwrap());
        }

        if device_extensions.vk_amd_shader_info {
            extensions.push(CString::new(&"VK_AMD_shader_info"[..]).unwrap());
        }

        if device_extensions.vk_amd_shader_trinary_minmax {
            extensions.push(CString::new(&"VK_AMD_shader_trinary_minmax"[..]).unwrap());
        }

        if device_extensions.vk_amd_texture_gather_bias_lod {
            extensions.push(CString::new(&"VK_AMD_texture_gather_bias_lod"[..]).unwrap());
        }

        if device_extensions.vk_android_external_memory_android_hardware_buffer {
            extensions.push(
                CString::new(&"VK_ANDROID_external_memory_android_hardware_buffer"[..]).unwrap(),
            );
        }

        if device_extensions.vk_ext_acquire_xlib_display {
            extensions.push(CString::new(&"VK_EXT_acquire_xlib_display"[..]).unwrap());
        }

        if device_extensions.vk_ext_astc_decode_mode {
            extensions.push(CString::new(&"VK_EXT_astc_decode_mode"[..]).unwrap());
        }

        if device_extensions.vk_ext_blend_operation_advanced {
            extensions.push(CString::new(&"VK_EXT_blend_operation_advanced"[..]).unwrap());
        }

        if device_extensions.vk_ext_buffer_device_address {
            extensions.push(CString::new(&"VK_EXT_buffer_device_address"[..]).unwrap());
        }

        if device_extensions.vk_ext_calibrated_timestamps {
            extensions.push(CString::new(&"VK_EXT_calibrated_timestamps"[..]).unwrap());
        }

        if device_extensions.vk_ext_conditional_rendering {
            extensions.push(CString::new(&"VK_EXT_conditional_rendering"[..]).unwrap());
        }

        if device_extensions.vk_ext_conservative_rasterization {
            extensions.push(CString::new(&"VK_EXT_conservative_rasterization"[..]).unwrap());
        }

        if device_extensions.vk_ext_custom_border_color {
            extensions.push(CString::new(&"VK_EXT_custom_border_color"[..]).unwrap());
        }

        if device_extensions.vk_ext_debug_marker {
            extensions.push(CString::new(&"VK_EXT_debug_marker"[..]).unwrap());
        }

        if device_extensions.vk_ext_debug_report {
            extensions.push(CString::new(&"VK_EXT_debug_report"[..]).unwrap());
        }

        if device_extensions.vk_ext_debug_utils {
            extensions.push(CString::new(&"VK_EXT_debug_utils"[..]).unwrap());
        }

        if device_extensions.vk_ext_depth_clip_enable {
            extensions.push(CString::new(&"VK_EXT_depth_clip_enable"[..]).unwrap());
        }

        if device_extensions.vk_ext_depth_range_unrestricted {
            extensions.push(CString::new(&"VK_EXT_depth_range_unrestricted"[..]).unwrap());
        }

        if device_extensions.vk_ext_descriptor_indexing {
            extensions.push(CString::new(&"VK_EXT_descriptor_indexing"[..]).unwrap());
        }

        if device_extensions.vk_ext_direct_mode_display {
            extensions.push(CString::new(&"VK_EXT_direct_mode_display"[..]).unwrap());
        }

        if device_extensions.vk_ext_discard_rectangles {
            extensions.push(CString::new(&"VK_EXT_discard_rectangles"[..]).unwrap());
        }

        if device_extensions.vk_ext_display_control {
            extensions.push(CString::new(&"VK_EXT_display_control"[..]).unwrap());
        }

        if device_extensions.vk_ext_display_surface_counter {
            extensions.push(CString::new(&"VK_EXT_display_surface_counter"[..]).unwrap());
        }

        if device_extensions.vk_ext_external_memory_dma_buf {
            extensions.push(CString::new(&"VK_EXT_external_memory_dma_buf"[..]).unwrap());
        }

        if device_extensions.vk_ext_external_memory_host {
            extensions.push(CString::new(&"VK_EXT_external_memory_host"[..]).unwrap());
        }

        if device_extensions.vk_ext_filter_cubic {
            extensions.push(CString::new(&"VK_EXT_filter_cubic"[..]).unwrap());
        }

        if device_extensions.vk_ext_fragment_density_map {
            extensions.push(CString::new(&"VK_EXT_fragment_density_map"[..]).unwrap());
        }

        if device_extensions.vk_ext_fragment_shader_interlock {
            extensions.push(CString::new(&"VK_EXT_fragment_shader_interlock"[..]).unwrap());
        }

        if device_extensions.vk_ext_full_screen_exclusive {
            extensions.push(CString::new(&"VK_EXT_full_screen_exclusive"[..]).unwrap());
        }

        if device_extensions.vk_ext_global_priority {
            extensions.push(CString::new(&"VK_EXT_global_priority_query"[..]).unwrap());
        }

        if device_extensions.vk_ext_hdr_metadata {
            extensions.push(CString::new(&"VK_EXT_hdr_metadata"[..]).unwrap());
        }

        if device_extensions.vk_ext_headless_surface {
            extensions.push(CString::new(&"VK_EXT_headless_surface"[..]).unwrap());
        }

        if device_extensions.vk_ext_host_query_reset {
            extensions.push(CString::new(&"VK_EXT_host_query_reset"[..]).unwrap());
        }

        if device_extensions.vk_ext_image_drm_format_modifier {
            extensions.push(CString::new(&"VK_EXT_image_drm_format_modifier"[..]).unwrap());
        }

        if device_extensions.vk_ext_index_type_uint8 {
            extensions.push(CString::new(&"VK_EXT_index_type_uint8"[..]).unwrap());
        }

        if device_extensions.vk_ext_inline_uniform_block {
            extensions.push(CString::new(&"VK_EXT_inline_uniform_block"[..]).unwrap());
        }

        if device_extensions.vk_ext_line_rasterization {
            extensions.push(CString::new(&"VK_EXT_line_rasterization"[..]).unwrap());
        }

        if device_extensions.vk_ext_memory_budget {
            extensions.push(CString::new(&"VK_EXT_memory_budget"[..]).unwrap());
        }

        if device_extensions.vk_ext_memory_priority {
            extensions.push(CString::new(&"VK_EXT_memory_priority"[..]).unwrap());
        }

        if device_extensions.vk_ext_metal_surface {
            extensions.push(CString::new(&"VK_EXT_metal_surface"[..]).unwrap());
        }

        if device_extensions.vk_ext_pci_bus_info {
            extensions.push(CString::new(&"VK_EXT_pci_bus_info"[..]).unwrap());
        }

        if device_extensions.vk_ext_pipeline_creation_cache_control {
            extensions.push(CString::new(&"VK_EXT_pipeline_creation_cache_control"[..]).unwrap());
        }

        if device_extensions.vk_ext_pipeline_creation_feedback {
            extensions.push(CString::new(&"VK_EXT_pipeline_creation_feedback"[..]).unwrap());
        }

        if device_extensions.vk_ext_post_depth_coverage {
            extensions.push(CString::new(&"VK_EXT_post_depth_coverage"[..]).unwrap());
        }

        if device_extensions.vk_ext_private_data {
            extensions.push(CString::new(&"VK_EXT_private_data"[..]).unwrap());
        }

        if device_extensions.vk_ext_queue_family_foreign {
            extensions.push(CString::new(&"VK_EXT_queue_family_foreign"[..]).unwrap());
        }

        if device_extensions.vk_ext_robustness2 {
            extensions.push(CString::new(&"VK_EXT_robustness2"[..]).unwrap());
        }

        if device_extensions.vk_ext_sample_locations {
            extensions.push(CString::new(&"VK_EXT_sample_locations"[..]).unwrap());
        }

        if device_extensions.vk_ext_sampler_filter_minmax {
            extensions.push(CString::new(&"VK_EXT_sampler_filter_minmax"[..]).unwrap());
        }

        if device_extensions.vk_ext_scalar_block_layout {
            extensions.push(CString::new(&"VK_EXT_scalar_block_layout"[..]).unwrap());
        }

        if device_extensions.vk_ext_separate_stencil_usage {
            extensions.push(CString::new(&"VK_EXT_separate_stencil_usage"[..]).unwrap());
        }

        if device_extensions.vk_ext_shader_demote_to_helper_invocation {
            extensions
                .push(CString::new(&"VK_EXT_shader_demote_to_helper_invocation"[..]).unwrap());
        }

        if device_extensions.vk_ext_shader_stencil_export {
            extensions.push(CString::new(&"VK_EXT_shader_stencil_export"[..]).unwrap());
        }

        if device_extensions.vk_ext_shader_subgroup_ballot {
            extensions.push(CString::new(&"VK_EXT_shader_subgroup_ballot"[..]).unwrap());
        }

        if device_extensions.vk_ext_shader_subgroup_vote {
            extensions.push(CString::new(&"VK_EXT_shader_subgroup_vote"[..]).unwrap());
        }

        if device_extensions.vk_ext_shader_viewport_index_layer {
            extensions.push(CString::new(&"VK_EXT_shader_viewport_index_layer"[..]).unwrap());
        }

        if device_extensions.vk_ext_subgroup_size_control {
            extensions.push(CString::new(&"VK_EXT_subgroup_size_control"[..]).unwrap());
        }

        if device_extensions.vk_ext_swapchain_colorspace {
            extensions.push(CString::new(&"VK_EXT_swapchain_colorspace"[..]).unwrap());
        }

        if device_extensions.vk_ext_texel_buffer_alignment {
            extensions.push(CString::new(&"VK_EXT_texel_buffer_alignment"[..]).unwrap());
        }

        if device_extensions.vk_ext_texture_compression_astc_hdr {
            extensions.push(CString::new(&"VK_EXT_texture_compression_astc_hdr"[..]).unwrap());
        }

        if device_extensions.vk_ext_tooling_info {
            extensions.push(CString::new(&"VK_EXT_tooling_info"[..]).unwrap());
        }

        if device_extensions.vk_ext_transform_feedback {
            extensions.push(CString::new(&"VK_EXT_transform_feedback"[..]).unwrap());
        }

        if device_extensions.vk_ext_validation_cache {
            extensions.push(CString::new(&"VK_EXT_validation_cache"[..]).unwrap());
        }

        if device_extensions.vk_ext_validation_features {
            extensions.push(CString::new(&"VK_EXT_validation_features"[..]).unwrap());
        }

        if device_extensions.vk_ext_validation_flags {
            extensions.push(CString::new(&"VK_EXT_validation_flags"[..]).unwrap());
        }

        if device_extensions.vk_ext_vertex_attribute_divisor {
            extensions.push(CString::new(&"VK_EXT_vertex_attribute_divisor"[..]).unwrap());
        }

        if device_extensions.vk_ext_ycbcr_image_arrays {
            extensions.push(CString::new(&"VK_EXT_ycbcr_image_arrays"[..]).unwrap());
        }

        if device_extensions.vk_fuchsia_imagepipe_surface {
            extensions.push(CString::new(&"VK_FUCHSIA_imagepipe_surface"[..]).unwrap());
        }

        if device_extensions.vk_ggp_frame_token {
            extensions.push(CString::new(&"VK_GGP_frame_token"[..]).unwrap());
        }

        if device_extensions.vk_ggp_stream_descriptor_surface {
            extensions.push(CString::new(&"VK_GGP_stream_descriptor_surface"[..]).unwrap());
        }

        if device_extensions.vk_google_decorate_string {
            extensions.push(CString::new(&"VK_GOOGLE_decorate_string"[..]).unwrap());
        }

        if device_extensions.vk_google_display_timing {
            extensions.push(CString::new(&"VK_GOOGLE_display_timing"[..]).unwrap());
        }

        if device_extensions.vk_google_hlsl_functionality1 {
            extensions.push(CString::new(&"VK_GOOGLE_hlsl_functionality1"[..]).unwrap());
        }

        if device_extensions.vk_google_user_type {
            extensions.push(CString::new(&"VK_GOOGLE_user_type"[..]).unwrap());
        }

        if device_extensions.vk_img_filter_cubic {
            extensions.push(CString::new(&"VK_IMG_filter_cubic"[..]).unwrap());
        }

        if device_extensions.vk_img_format_pvrtc {
            extensions.push(CString::new(&"VK_IMG_format_pvrtc"[..]).unwrap());
        }

        if device_extensions.vk_intel_performance_query {
            extensions.push(CString::new(&"VK_INTEL_performance_query"[..]).unwrap());
        }

        if device_extensions.vk_intel_shader_integer_functions2 {
            extensions.push(CString::new(&"VK_INTEL_shader_integer_functions2"[..]).unwrap());
        }

        if device_extensions.vk_khr_16bit_storage {
            extensions.push(CString::new(&"VK_KHR_16bit_storage"[..]).unwrap());
        }

        if device_extensions.vk_khr_8bit_storage {
            extensions.push(CString::new(&"VK_KHR_8bit_storage"[..]).unwrap());
        }
        if device_extensions.vk_khr_acceleration_structure {
            extensions.push(CString::new(&"VK_KHR_acceleration_structure"[..]).unwrap());
        }
        if device_extensions.vk_khr_android_surface {
            extensions.push(CString::new(&"VK_KHR_android_surface"[..]).unwrap());
        }

        if device_extensions.vk_khr_bind_memory2 {
            extensions.push(CString::new(&"VK_KHR_bind_memory2"[..]).unwrap());
        }

        if device_extensions.vk_khr_buffer_device_address {
            extensions.push(CString::new(&"VK_KHR_buffer_device_address"[..]).unwrap());
        }

        if device_extensions.vk_khr_create_renderpass2 {
            extensions.push(CString::new(&"VK_KHR_create_renderpass2"[..]).unwrap());
        }

        if device_extensions.vk_khr_dedicated_allocation {
            extensions.push(CString::new(&"VK_KHR_dedicated_allocation"[..]).unwrap());
        }

        if device_extensions.vk_khr_deferred_host_operations {
            extensions.push(CString::new(&"VK_KHR_deferred_host_operations"[..]).unwrap());
        }

        if device_extensions.vk_khr_depth_stencil_resolve {
            extensions.push(CString::new(&"VK_KHR_depth_stencil_resolve"[..]).unwrap());
        }

        if device_extensions.vk_khr_descriptor_update_template {
            extensions.push(CString::new(&"VK_KHR_descriptor_update_template"[..]).unwrap());
        }

        if device_extensions.vk_khr_device_group {
            extensions.push(CString::new(&"VK_KHR_device_group"[..]).unwrap());
        }

        if device_extensions.vk_khr_device_group_creation {
            extensions.push(CString::new(&"VK_KHR_device_group_creation"[..]).unwrap());
        }

        if device_extensions.vk_khr_display {
            extensions.push(CString::new(&"VK_KHR_display"[..]).unwrap());
        }

        if device_extensions.vk_khr_display_swapchain {
            extensions.push(CString::new(&"VK_KHR_display_swapchain"[..]).unwrap());
        }

        if device_extensions.vk_khr_draw_indirect_count {
            extensions.push(CString::new(&"VK_KHR_draw_indirect_count"[..]).unwrap());
        }

        if device_extensions.vk_khr_driver_properties {
            extensions.push(CString::new(&"VK_KHR_driver_properties"[..]).unwrap());
        }

        if device_extensions.vk_khr_dynamic_rendering {
            extensions.push(CString::new(&"VK_KHR_dynamic_rendering"[..]).unwrap());
        }

        if device_extensions.vk_khr_external_fence {
            extensions.push(CString::new(&"VK_KHR_external_fence"[..]).unwrap());
        }

        if device_extensions.vk_khr_external_fence_capabilities {
            extensions.push(CString::new(&"VK_KHR_external_fence_capabilities"[..]).unwrap());
        }

        if device_extensions.vk_khr_external_fence_fd {
            extensions.push(CString::new(&"VK_KHR_external_fence_fd"[..]).unwrap());
        }

        if device_extensions.vk_khr_external_fence_win32 {
            extensions.push(CString::new(&"VK_KHR_external_fence_win32"[..]).unwrap());
        }

        if device_extensions.vk_khr_external_memory {
            extensions.push(CString::new(&"VK_KHR_external_memory"[..]).unwrap());
        }

        if device_extensions.vk_khr_external_memory_capabilities {
            extensions.push(CString::new(&"VK_KHR_external_memory_capabilities"[..]).unwrap());
        }

        if device_extensions.vk_khr_external_memory_fd {
            extensions.push(CString::new(&"VK_KHR_external_memory_fd"[..]).unwrap());
        }

        if device_extensions.vk_khr_external_memory_win32 {
            extensions.push(CString::new(&"VK_KHR_external_memory_win32"[..]).unwrap());
        }

        if device_extensions.vk_khr_external_semaphore {
            extensions.push(CString::new(&"VK_KHR_external_semaphore"[..]).unwrap());
        }

        if device_extensions.vk_khr_external_semaphore_capabilities {
            extensions.push(CString::new(&"VK_KHR_external_semaphore_capabilities"[..]).unwrap());
        }

        if device_extensions.vk_khr_external_semaphore_fd {
            extensions.push(CString::new(&"VK_KHR_external_semaphore_fd"[..]).unwrap());
        }

        if device_extensions.vk_khr_external_semaphore_win32 {
            extensions.push(CString::new(&"VK_KHR_external_semaphore_win32"[..]).unwrap());
        }

        if device_extensions.vk_khr_get_display_properties2 {
            extensions.push(CString::new(&"VK_KHR_get_display_properties2"[..]).unwrap());
        }

        if device_extensions.vk_khr_get_memory_requirements2 {
            extensions.push(CString::new(&"VK_KHR_get_memory_requirements2"[..]).unwrap());
        }

        if device_extensions.vk_khr_get_physical_device_properties2 {
            extensions.push(CString::new(&"VK_KHR_get_physical_device_properties2"[..]).unwrap());
        }

        if device_extensions.vk_khr_get_surface_capabilities2 {
            extensions.push(CString::new(&"VK_KHR_get_surface_capabilities2"[..]).unwrap());
        }

        if device_extensions.vk_khr_image_format_list {
            extensions.push(CString::new(&"VK_KHR_image_format_list"[..]).unwrap());
        }

        if device_extensions.vk_khr_imageless_framebuffer {
            extensions.push(CString::new(&"VK_KHR_imageless_framebuffer"[..]).unwrap());
        }

        if device_extensions.vk_khr_incremental_present {
            extensions.push(CString::new(&"VK_KHR_incremental_present"[..]).unwrap());
        }

        if device_extensions.vk_khr_maintenance1 {
            extensions.push(CString::new(&"VK_KHR_maintenance1"[..]).unwrap());
        }

        if device_extensions.vk_khr_maintenance2 {
            extensions.push(CString::new(&"VK_KHR_maintenance2"[..]).unwrap());
        }

        if device_extensions.vk_khr_maintenance3 {
            extensions.push(CString::new(&"VK_KHR_maintenance3"[..]).unwrap());
        }

        if device_extensions.vk_khr_multiview {
            extensions.push(CString::new(&"VK_KHR_performance_query"[..]).unwrap());
        }

        if device_extensions.vk_khr_performance_query {
            extensions.push(CString::new(&"VK_KHR_performance_query"[..]).unwrap());
        }

        if device_extensions.vk_khr_pipeline_executable_properties {
            extensions.push(CString::new(&"VK_KHR_pipeline_executable_properties"[..]).unwrap());
        }

        if device_extensions.vk_khr_pipeline_library {
            extensions.push(CString::new(&"VK_KHR_pipeline_library"[..]).unwrap());
        }

        if device_extensions.vk_khr_push_descriptor {
            extensions.push(CString::new(&"VK_KHR_push_descriptor"[..]).unwrap());
        }

        if device_extensions.vk_khr_ray_tracing_pipeline {
            extensions.push(CString::new(&"VK_KHR_ray_tracing_pipeline"[..]).unwrap());
        }

        if device_extensions.vk_khr_ray_query {
            extensions.push(CString::new(&"VK_KHR_ray_query"[..]).unwrap());
        }

        if device_extensions.vk_khr_relaxed_block_layout {
            extensions.push(CString::new(&"VK_KHR_relaxed_block_layout"[..]).unwrap());
        }

        if device_extensions.vk_khr_sampler_mirror_clamp_to_edge {
            extensions.push(CString::new(&"VK_KHR_sampler_mirror_clamp_to_edge"[..]).unwrap());
        }

        if device_extensions.vk_khr_sampler_ycbcr_conversion {
            extensions.push(CString::new(&"VK_KHR_sampler_ycbcr_conversion"[..]).unwrap());
        }

        if device_extensions.vk_khr_separate_depth_stencil_layouts {
            extensions.push(CString::new(&"VK_KHR_separate_depth_stencil_layouts"[..]).unwrap());
        }

        if device_extensions.vk_khr_shader_atomic_int64 {
            extensions.push(CString::new(&"VK_KHR_shader_atomic_int64"[..]).unwrap());
        }

        if device_extensions.vk_khr_shader_clock {
            extensions.push(CString::new(&"VK_KHR_shader_clock"[..]).unwrap());
        }

        if device_extensions.vk_khr_shader_draw_parameters {
            extensions.push(CString::new(&"VK_KHR_shader_draw_parameters"[..]).unwrap());
        }

        if device_extensions.vk_khr_shader_float16_int8 {
            extensions.push(CString::new(&"VK_KHR_shader_float16_int8"[..]).unwrap());
        }

        if device_extensions.vk_khr_shader_float_controls {
            extensions.push(CString::new(&"VK_KHR_shader_float_controls"[..]).unwrap());
        }

        if device_extensions.vk_khr_shader_non_semantic_info {
            extensions.push(CString::new(&"VK_KHR_shader_non_semantic_info"[..]).unwrap());
        }

        if device_extensions.vk_khr_shader_subgroup_extended_types {
            extensions.push(CString::new(&"VK_KHR_shader_subgroup_extended_types"[..]).unwrap());
        }

        if device_extensions.vk_khr_shared_presentable_image {
            extensions.push(CString::new(&"VK_KHR_shared_presentable_image"[..]).unwrap());
        }

        if device_extensions.vk_khr_spirv_1_4 {
            extensions.push(CString::new(&"VK_KHR_spirv_1_4"[..]).unwrap());
        }

        if device_extensions.vk_khr_storage_buffer_storage_class {
            extensions.push(CString::new(&"VK_KHR_storage_buffer_storage_class"[..]).unwrap());
        }

        if device_extensions.vk_khr_surface {
            extensions.push(CString::new(&"VK_KHR_surface"[..]).unwrap());
        }

        if device_extensions.vk_khr_surface_protected_capabilities {
            extensions.push(CString::new(&"VK_KHR_surface_protected_capabilities"[..]).unwrap());
        }

        if device_extensions.vk_khr_swapchain {
            extensions.push(CString::new(&"VK_KHR_swapchain"[..]).unwrap());
        }

        if device_extensions.vk_khr_swapchain_mutable_format {
            extensions.push(CString::new(&"VK_KHR_swapchain_mutable_format"[..]).unwrap());
        }

        if device_extensions.vk_khr_timeline_semaphore {
            extensions.push(CString::new(&"VK_KHR_timeline_semaphore"[..]).unwrap());
        }

        if device_extensions.vk_khr_uniform_buffer_standard_layout {
            extensions.push(CString::new(&"VK_KHR_uniform_buffer_standard_layout"[..]).unwrap());
        }

        if device_extensions.vk_khr_variable_pointers {
            extensions.push(CString::new(&"VK_KHR_variable_pointers"[..]).unwrap());
        }

        if device_extensions.vk_khr_vulkan_memory_model {
            extensions.push(CString::new(&"VK_KHR_vulkan_memory_model"[..]).unwrap());
        }

        if device_extensions.vk_khr_wayland_surface {
            extensions.push(CString::new(&"VK_KHR_wayland_surface"[..]).unwrap());
        }

        if device_extensions.vk_khr_win32_keyed_mutex {
            extensions.push(CString::new(&"VK_KHR_win32_keyed_mutex"[..]).unwrap());
        }

        if device_extensions.vk_khr_win32_surface {
            extensions.push(CString::new(&"VK_KHR_win32_surface"[..]).unwrap());
        }

        if device_extensions.vk_khr_xcb_surface {
            extensions.push(CString::new(&"VK_KHR_xcb_surface"[..]).unwrap());
        }

        if device_extensions.vk_khr_xlib_surface {
            extensions.push(CString::new(&"VK_KHR_xlib_surface"[..]).unwrap());
        }

        if device_extensions.vk_mvk_ios_surface {
            extensions.push(CString::new(&"VK_MVK_ios_surface"[..]).unwrap());
        }

        if device_extensions.vk_mvk_macos_surface {
            extensions.push(CString::new(&"VK_MVK_macos_surface"[..]).unwrap());
        }

        if device_extensions.vk_nn_vi_surface {
            extensions.push(CString::new(&"VK_NN_vi_surface"[..]).unwrap());
        }

        if device_extensions.vk_nvx_image_view_handle {
            extensions.push(CString::new(&""[..]).unwrap());
        }

        if device_extensions.vk_nvx_multiview_per_view_attributes {
            extensions.push(CString::new(&"VK_NVX_image_view_handle"[..]).unwrap());
        }

        if device_extensions.vk_nv_clip_space_w_scaling {
            extensions.push(CString::new(&"VK_NV_clip_space_w_scaling"[..]).unwrap());
        }

        if device_extensions.vk_nv_compute_shader_derivatives {
            extensions.push(CString::new(&"VK_NV_compute_shader_derivatives"[..]).unwrap());
        }

        if device_extensions.vk_nv_cooperative_matrix {
            extensions.push(CString::new(&"VK_NV_cooperative_matrix"[..]).unwrap());
        }

        if device_extensions.vk_nv_corner_sampled_image {
            extensions.push(CString::new(&"VK_NV_corner_sampled_image"[..]).unwrap());
        }

        if device_extensions.vk_nv_coverage_reduction_mode {
            extensions.push(CString::new(&"VK_NV_coverage_reduction_mode"[..]).unwrap());
        }

        if device_extensions.vk_nv_dedicated_allocation {
            extensions.push(CString::new(&"VK_NV_dedicated_allocation"[..]).unwrap());
        }

        if device_extensions.vk_nv_dedicated_allocation_image_aliasing {
            extensions
                .push(CString::new(&"VK_NV_dedicated_allocation_image_aliasing"[..]).unwrap());
        }

        if device_extensions.vk_nv_device_diagnostic_checkpoints {
            extensions.push(CString::new(&"VK_NV_device_diagnostic_checkpoints"[..]).unwrap());
        }

        if device_extensions.vk_nv_device_diagnostics_config {
            extensions.push(CString::new(&"VK_NV_device_diagnostics_config"[..]).unwrap());
        }

        if device_extensions.vk_nv_device_generated_commands {
            extensions.push(CString::new(&"VK_NV_device_generated_commands"[..]).unwrap());
        }

        if device_extensions.vk_nv_external_memory {
            extensions.push(CString::new(&"VK_NV_external_memory"[..]).unwrap());
        }

        if device_extensions.vk_nv_external_memory_capabilities {
            extensions.push(CString::new(&"VK_NV_external_memory_capabilities"[..]).unwrap());
        }

        if device_extensions.vk_nv_external_memory_win32 {
            extensions.push(CString::new(&"VK_NV_external_memory_win32"[..]).unwrap());
        }

        if device_extensions.vk_nv_fill_rectangle {
            extensions.push(CString::new(&"VK_NV_fill_rectangle"[..]).unwrap());
        }

        if device_extensions.vk_nv_fragment_coverage_to_color {
            extensions.push(CString::new(&"VK_NV_fragment_coverage_to_color"[..]).unwrap());
        }

        if device_extensions.vk_nv_fragment_shader_barycentric {
            extensions.push(CString::new(&"VK_NV_fragment_shader_barycentric"[..]).unwrap());
        }

        if device_extensions.vk_nv_framebuffer_mixed_samples {
            extensions.push(CString::new(&"VK_NV_framebuffer_mixed_samples"[..]).unwrap());
        }

        if device_extensions.vk_nv_geometry_shader_passthrough {
            extensions.push(CString::new(&"VK_NV_geometry_shader_passthrough"[..]).unwrap());
        }

        if device_extensions.vk_nv_glsl_shader {
            extensions.push(CString::new(&"VK_NV_glsl_shader"[..]).unwrap());
        }

        if device_extensions.vk_nv_mesh_shader {
            extensions.push(CString::new(&"VK_NV_mesh_shader"[..]).unwrap());
        }

        if device_extensions.vk_nv_ray_tracing {
            extensions.push(CString::new(&"VK_NV_ray_tracing"[..]).unwrap());
        }

        if device_extensions.vk_nv_representative_fragment_test {
            extensions.push(CString::new(&"VK_NV_representative_fragment_test"[..]).unwrap());
        }

        if device_extensions.vk_nv_sample_mask_override_coverage {
            extensions.push(CString::new(&"VK_NV_sample_mask_override_coverage"[..]).unwrap());
        }

        if device_extensions.vk_nv_scissor_exclusive {
            extensions.push(CString::new(&"VK_NV_scissor_exclusive"[..]).unwrap());
        }

        if device_extensions.vk_nv_shader_image_footprint {
            extensions.push(CString::new(&"VK_NV_shader_image_footprint"[..]).unwrap());
        }

        if device_extensions.vk_nv_shader_sm_builtins {
            extensions.push(CString::new(&"VK_NV_shader_sm_builtins"[..]).unwrap());
        }

        if device_extensions.vk_nv_shader_subgroup_partitioned {
            extensions.push(CString::new(&"VK_NV_shader_subgroup_partitioned"[..]).unwrap());
        }

        if device_extensions.vk_nv_shading_rate_image {
            extensions.push(CString::new(&"VK_NV_shading_rate_image"[..]).unwrap());
        }

        if device_extensions.vk_nv_viewport_array2 {
            extensions.push(CString::new(&"VK_NV_viewport_array2"[..]).unwrap());
        }

        if device_extensions.vk_nv_viewport_swizzle {
            extensions.push(CString::new(&"VK_NV_viewport_swizzle"[..]).unwrap());
        }

        if device_extensions.vk_nv_win32_keyed_mutex {
            extensions.push(CString::new(&"VK_NV_win32_keyed_mutex"[..]).unwrap());
        }

        if device_extensions.vk_qcom_render_pass_shader_resolve {
            extensions.push(CString::new(&""[..]).unwrap());
        }

        if device_extensions.vk_qcom_render_pass_store_ops {
            extensions.push(CString::new(&"VK_QCOM_render_pass_store_ops"[..]).unwrap());
        }

        if device_extensions.vk_qcom_render_pass_transform {
            extensions.push(CString::new(&"VK_QCOM_render_pass_transform"[..]).unwrap());
        }

        if device_extensions.wsitypes {
            extensions.push(CString::new(&""[..]).unwrap());
        }

        extensions
    }
}
