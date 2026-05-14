#####################################################################################
# Examples/Integration tests setup
#####################################################################################
# Include additional examples here
add_subdirectory(${CMAKE_CURRENT_SOURCE_DIR}/greeting)

# Target to run all examples
add_custom_target(run_examples)

# Add their execution target here
add_dependencies(run_examples run_greeting)
