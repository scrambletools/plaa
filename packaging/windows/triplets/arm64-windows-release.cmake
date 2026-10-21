# vcpkg's arm64-windows, release builds only: DLLs on the dynamic C
# runtime, for the libraries plaa's Windows packages carry.
set(VCPKG_TARGET_ARCHITECTURE arm64)
set(VCPKG_CRT_LINKAGE dynamic)
set(VCPKG_LIBRARY_LINKAGE dynamic)
set(VCPKG_BUILD_TYPE release)
