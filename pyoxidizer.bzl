# This file defines how PyOxidizer application building and packaging is
# performed. See the pyoxidizer crate's documentation for extensive
# documentation on this file format.

def make_dist():
    return default_python_distribution(flavor='standalone_dynamic', python_version='3.8')

def make_exe(dist):
    policy = dist.make_python_packaging_policy()
    policy.allow_files = True
    policy.bytecode_optimize_level_one = True
    policy.resources_location_fallback = "filesystem-relative:lib"

    python_config = dist.make_python_interpreter_config()
    python_config.run_command = "import chip8.app; chip8.app.main()"

    exe = dist.to_python_executable(
        name="emu-chip-8",
        packaging_policy=policy,
        config=python_config,
    )

    for resource in exe.read_virtualenv(CWD + "/venv"):
        if resource.name.startswith('.dylibs'):
            resource.add_include = True
            resource.add_location = "filesystem-relative:lib"
        exe.add_python_resource(resource)

    return exe

def make_embedded_resources(exe):
    return exe.to_embedded_resources()

def make_install(exe):
    files = FileManifest()
    files.add_python_resource(".", exe)
    return files


register_target("dist", make_dist)
register_target("exe", make_exe, depends=["dist"])
register_target("resources", make_embedded_resources, depends=["exe"], default_build_script=True)
register_target("install", make_install, depends=["exe"], default=True)

resolve_targets()

# END OF COMMON USER-ADJUSTED SETTINGS.
#
# Everything below this is typically managed by PyOxidizer and doesn't need
# to be updated by people.

PYOXIDIZER_VERSION = "0.10.3"
PYOXIDIZER_COMMIT = "UNKNOWN"
