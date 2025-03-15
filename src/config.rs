/**********************************************************************************************
*
*   raylib configuration flags
*
*   This file defines all the configuration flags for the different raylib modules
*
*   LICENSE: zlib/libpng
*
*   Copyright (c) 2018-2025 Ahmad Fatoum & Ramon Santamaria (@raysan5)
*
*   This software is provided "as-is", without any express or implied warranty. In no event
*   will the authors be held liable for any damages arising from the use of this software.
*
*   Permission is granted to anyone to use this software for any purpose, including commercial
*   applications, and to alter it and redistribute it freely, subject to the following restrictions:
*
*     1. The origin of this software must not be misrepresented; you must not claim that you
*     wrote the original software. If you use this software in a product, an acknowledgment
*     in the product documentation would be appreciated but is not required.
*
*     2. Altered source versions must be plainly marked as such, and must not be misrepresented
*     as being the original software.
*
*     3. This notice may not be removed or altered from any source distribution.
*
**********************************************************************************************/

#[cfg(feature = "support_module_raudio")]
use miniaudio_rs as ma;

/// Raylib Configuration values
pub trait RaylibConfig {
    type RCoreCfg: RCoreConfig;
    const RCORE: Self::RCoreCfg;

    type RlglCfg: RlglConfig;
    const RLGL: Self::RlglCfg;

    #[cfg(feature = "support_module_rshapes")]
    type RShapesCfg: RShapesConfig;
    #[cfg(feature = "support_module_rshapes")]
    const RSHAPES: Self::RShapesCfg;

    #[cfg(feature = "support_module_rtext")]
    type RTextCfg: RTextConfig;
    #[cfg(feature = "support_module_rtext")]
    const RTEXT: Self::RTextCfg;

    #[cfg(feature = "support_module_raudio")]
    type RAudioCfg: RAudioConfig;
    #[cfg(feature = "support_module_raudio")]
    const RAUDIO: Self::RAudioCfg;

    type UtilsCfg: UtilsConfig;
    const UTILS: Self::UtilsCfg;
}

/// rcore: Configuration values
pub trait RCoreConfig {
    /// Maximum file paths capacity
    const MAX_FILEPATH_CAPACITY:    usize =  8192;
    /// Maximum length for filepaths (Linux PATH_MAX default value)
    const MAX_FILEPATH_LENGTH:      usize =  4096;

    /// Maximum number of keyboard keys supported
    const MAX_KEYBOARD_KEYS:        usize =   512;
    /// Maximum number of mouse buttons supported
    const MAX_MOUSE_BUTTONS:        usize =     8;
    /// Maximum number of gamepads supported
    const MAX_GAMEPADS:             usize =     4;
    /// Maximum number of axis supported (per gamepad)
    const MAX_GAMEPAD_AXIS:         usize =     8;
    /// Maximum number of buttons supported (per gamepad)
    const MAX_GAMEPAD_BUTTONS:      usize =    32;
    /// Maximum vibration time in seconds
    const MAX_GAMEPAD_VIBRATION_TIME: f32 =     2.0;
    /// Maximum number of touch points supported
    const MAX_TOUCH_POINTS:         usize =     8;
    /// Maximum number of keys in the key input queue
    const MAX_KEY_PRESSED_QUEUE:    usize =    16;
    /// Maximum number of characters in the char input queue
    const MAX_CHAR_PRESSED_QUEUE:   usize =    16;

    /// Max size allocated for decompression in MB
    const MAX_DECOMPRESSION_SIZE:   usize =    64;

    /// Maximum number of automation events to record
    const MAX_AUTOMATION_EVENTS:    usize = 16384;
}

/// Module: rlgl - Configuration values
pub trait RlglConfig {
    /// Default internal render batch elements limits
    const RL_DEFAULT_BATCH_BUFFER_ELEMENTS:     usize = 4096;
    /// Default number of batch buffers (multi-buffering)
    const RL_DEFAULT_BATCH_BUFFERS:             usize = 1;
    /// Default number of batch draw calls (by state changes: mode, texture)
    const RL_DEFAULT_BATCH_DRAWCALLS:           usize = 256;
    /// Maximum number of textures units that can be activated on batch drawing (SetShaderValueTexture())
    const RL_DEFAULT_BATCH_MAX_TEXTURE_UNITS:   usize = 4;

    /// Maximum size of internal Matrix stack
    const RL_MAX_MATRIX_STACK_SIZE: usize = 32;

    /// Maximum number of shader locations supported
    const RL_MAX_SHADER_LOCATIONS:  usize = 32;

    /// Default projection matrix near cull distance
    const RL_CULL_DISTANCE_NEAR:    f64 =    0.01;
    /// Default projection matrix far cull distance
    const RL_CULL_DISTANCE_FAR:     f64 = 1000.0;

    // Default shader vertex attribute locations
    const RL_DEFAULT_SHADER_ATTRIB_LOCATION_POSITION:    usize = 0;
    const RL_DEFAULT_SHADER_ATTRIB_LOCATION_TEXCOORD:    usize = 1;
    const RL_DEFAULT_SHADER_ATTRIB_LOCATION_NORMAL:      usize = 2;
    const RL_DEFAULT_SHADER_ATTRIB_LOCATION_COLOR:       usize = 3;
    const RL_DEFAULT_SHADER_ATTRIB_LOCATION_TANGENT:     usize = 4;
    const RL_DEFAULT_SHADER_ATTRIB_LOCATION_TEXCOORD2:   usize = 5;
    const RL_DEFAULT_SHADER_ATTRIB_LOCATION_INDICES:     usize = 6;
    #[cfg(feature = "rl_support_mesh_gpu_skinning")]
    const RL_DEFAULT_SHADER_ATTRIB_LOCATION_BONEIDS:     usize = 7;
    #[cfg(feature = "rl_support_mesh_gpu_skinning")]
    const RL_DEFAULT_SHADER_ATTRIB_LOCATION_BONEWEIGHTS: usize = 8;
    const RL_DEFAULT_SHADER_ATTRIB_LOCATION_INSTANCE_TX: usize = 9;


    // Default shader vertex attribute names to set location points
    // NOTE: When a new shader is loaded, the following locations are tried to be set for convenience
    /// Bound by default to shader location: RL_DEFAULT_SHADER_ATTRIB_LOCATION_POSITION
    const RL_DEFAULT_SHADER_ATTRIB_NAME_POSITION:  &str = "vertexPosition";
    /// Bound by default to shader location: RL_DEFAULT_SHADER_ATTRIB_LOCATION_TEXCOORD
    const RL_DEFAULT_SHADER_ATTRIB_NAME_TEXCOORD:  &str = "vertexTexCoord";
    /// Bound by default to shader location: RL_DEFAULT_SHADER_ATTRIB_LOCATION_NORMAL
    const RL_DEFAULT_SHADER_ATTRIB_NAME_NORMAL:    &str = "vertexNormal";
    /// Bound by default to shader location: RL_DEFAULT_SHADER_ATTRIB_LOCATION_COLOR
    const RL_DEFAULT_SHADER_ATTRIB_NAME_COLOR:     &str = "vertexColor";
    /// Bound by default to shader location: RL_DEFAULT_SHADER_ATTRIB_LOCATION_TANGENT
    const RL_DEFAULT_SHADER_ATTRIB_NAME_TANGENT:   &str = "vertexTangent";
    /// Bound by default to shader location: RL_DEFAULT_SHADER_ATTRIB_LOCATION_TEXCOORD2
    const RL_DEFAULT_SHADER_ATTRIB_NAME_TEXCOORD2: &str = "vertexTexCoord2";

    /// model-view-projection matrix
    const RL_DEFAULT_SHADER_UNIFORM_NAME_MVP:        &str = "mvp";
    /// view matrix
    const RL_DEFAULT_SHADER_UNIFORM_NAME_VIEW:       &str = "matView";
    /// projection matrix
    const RL_DEFAULT_SHADER_UNIFORM_NAME_PROJECTION: &str = "matProjection";
    /// model matrix
    const RL_DEFAULT_SHADER_UNIFORM_NAME_MODEL:      &str = "matModel";
    /// normal matrix (transpose(inverse(matModelView))
    const RL_DEFAULT_SHADER_UNIFORM_NAME_NORMAL:     &str = "matNormal";
    /// color diffuse (base tint color, multiplied by texture color)
    const RL_DEFAULT_SHADER_UNIFORM_NAME_COLOR:      &str = "colDiffuse";
    /// texture0 (texture slot active 0)
    const RL_DEFAULT_SHADER_SAMPLER2D_NAME_TEXTURE0: &str = "texture0";
    /// texture1 (texture slot active 1)
    const RL_DEFAULT_SHADER_SAMPLER2D_NAME_TEXTURE1: &str = "texture1";
    /// texture2 (texture slot active 2)
    const RL_DEFAULT_SHADER_SAMPLER2D_NAME_TEXTURE2: &str = "texture2";
}

#[cfg(feature = "support_module_rshapes")]
/// rshapes: Configuration values
pub trait RShapesConfig {
    /// Spline segments subdivisions
    const SPLINE_SEGMENT_DIVISIONS: usize = 24;
}

#[cfg(feature = "support_module_rtext")]
/// rtext: Configuration values
pub trait RTextConfig {
    /// Size of internal static buffers used on some functions:
    /// TextFormat(), TextSubtext(), TextToUpper(), TextToLower(), TextToPascal(), TextSplit()
    const MAX_TEXT_BUFFER_LENGTH: usize = 1024;

    /// Maximum number of substrings to split: TextSplit()
    const MAX_TEXTSPLIT_COUNT:    usize = 128;
}

/// rmodels: Configuration values
pub trait RModelsConfig {
    /// Maximum number of shader maps supported
    const MAX_MATERIAL_MAPS: usize = 12;

    #[cfg(feature = "rl_support_mesh_gpu_skinning")]
    /// Maximum vertex buffers (VBO) per mesh
    const MAX_MESH_VERTEX_BUFFERS: usize = 9;
    #[cfg(not(feature = "rl_support_mesh_gpu_skinning"))]
    /// Maximum vertex buffers (VBO) per mesh
    const MAX_MESH_VERTEX_BUFFERS: usize = 7;
}

#[cfg(feature = "support_module_raudio")]
/// raudio: Configuration values
#[allow(non_camel_case_types)]
pub trait RAudioConfig {
    // Device output format (miniaudio: float-32bit)
    const AUDIO_DEVICE_FORMAT: ma::Format = ma::Format::F32;
    /// Device output channels: stereo
    const AUDIO_DEVICE_CHANNELS:    usize = 2;
    /// Device sample rate (device default)
    const AUDIO_DEVICE_SAMPLE_RATE: usize = 0;

    /// Maximum number of audio pool channels
    const MAX_AUDIO_BUFFER_POOL_CHANNELS: usize = 16;
}

/// utils: Configuration values
pub trait UtilsConfig {
    /// Max length of one trace-log message
    const MAX_TRACELOG_MSG_LENGTH: usize = 256;
}
