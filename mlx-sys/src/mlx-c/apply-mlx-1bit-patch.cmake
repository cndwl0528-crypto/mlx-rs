cmake_minimum_required(VERSION 3.16)

if(NOT DEFINED MLX_SOURCE_DIR OR NOT DEFINED MLX_1BIT_PATCH_FILE)
  message(FATAL_ERROR
    "MLX_SOURCE_DIR and MLX_1BIT_PATCH_FILE are required for the MLX 1-bit patch")
endif()

set(_patch_include "mlx/backend/metal/kernels/quantized*")

execute_process(
  COMMAND git apply --check "--include=${_patch_include}" "${MLX_1BIT_PATCH_FILE}"
  WORKING_DIRECTORY "${MLX_SOURCE_DIR}"
  RESULT_VARIABLE _check_result
  OUTPUT_VARIABLE _check_output
  ERROR_VARIABLE _check_error)

if(_check_result EQUAL 0)
  execute_process(
    COMMAND git apply "--include=${_patch_include}" "${MLX_1BIT_PATCH_FILE}"
    WORKING_DIRECTORY "${MLX_SOURCE_DIR}"
    RESULT_VARIABLE _apply_result
    OUTPUT_VARIABLE _apply_output
    ERROR_VARIABLE _apply_error)
  if(NOT _apply_result EQUAL 0)
    message(FATAL_ERROR
      "Failed to apply MLX 1-bit affine kernel patch:\n${_apply_output}${_apply_error}")
  endif()
  message(STATUS "Applied MLX 1-bit affine kernel patch")
elseif(_check_result)
  execute_process(
    COMMAND git apply --reverse --check "--include=${_patch_include}" "${MLX_1BIT_PATCH_FILE}"
    WORKING_DIRECTORY "${MLX_SOURCE_DIR}"
    RESULT_VARIABLE _reverse_check_result
    OUTPUT_VARIABLE _reverse_check_output
    ERROR_VARIABLE _reverse_check_error)
  if(_reverse_check_result EQUAL 0)
    message(STATUS "MLX 1-bit affine kernel patch already applied")
  else()
    message(FATAL_ERROR
      "MLX 1-bit affine kernel patch is neither applicable nor already applied:\n"
      "forward check:\n${_check_output}${_check_error}\n"
      "reverse check:\n${_reverse_check_output}${_reverse_check_error}")
  endif()
else()
  message(FATAL_ERROR "git apply --check failed without a result code")
endif()
