import bpy
import math

# Clear scene
bpy.ops.wm.read_factory_settings(use_empty=True)

scene = bpy.context.scene
scene.render.resolution_x = 1920
scene.render.resolution_y = 1080
scene.render.fps = 60
import sys
argv = sys.argv
if "--" in argv:
    args = argv[argv.index("--") + 1:]
    if "--start" in args:
        scene.frame_start = int(args[args.index("--start") + 1])
    if "--end" in args:
        scene.frame_end = int(args[args.index("--end") + 1])
    if "--output" in args:
        scene.render.filepath = args[args.index("--output") + 1]
else:
    scene.frame_start = 0
    scene.frame_end = 600
scene.render.image_settings.file_format = 'PNG'

# Camera setup
cam_data = bpy.data.cameras.new(name='Camera')
cam_obj = bpy.data.objects.new(name='Camera', object_data=cam_data)
scene.collection.objects.link(cam_obj)
scene.camera = cam_obj
cam_obj.location = (0, 0, 10)
cam_data.type = 'ORTHO'
cam_data.ortho_scale = 19.2

# Layer: Image_hook_0
bpy.ops.mesh.primitive_plane_add(size=1)
obj = bpy.context.active_object
obj.name = 'Image_hook_0'
obj.location.x = 0
obj.location.y = 0

# Layer: Text_hook_1
bpy.ops.object.text_add()
obj = bpy.context.active_object
obj.name = 'Text_hook_1'
obj.data.body = 'The Digital Artisan'

# Layer: Text_hook_2
bpy.ops.object.text_add()
obj = bpy.context.active_object
obj.name = 'Text_hook_2'
obj.data.body = 'Building Videos with Rust'

# Layer: Image_bridge_0
bpy.ops.mesh.primitive_plane_add(size=1)
obj = bpy.context.active_object
obj.name = 'Image_bridge_0'
obj.location.x = 0
obj.location.y = 0

# Layer: Text_bridge_1
bpy.ops.object.text_add()
obj = bpy.context.active_object
obj.name = 'Text_bridge_1'
obj.data.body = 'The Rendering Pipeline'

# Layer: Image_payoff_0
bpy.ops.mesh.primitive_plane_add(size=1)
obj = bpy.context.active_object
obj.name = 'Image_payoff_0'
obj.location.x = 0
obj.location.y = 0

# Layer: Text_payoff_1
bpy.ops.object.text_add()
obj = bpy.context.active_object
obj.name = 'Text_payoff_1'
obj.data.body = 'Fast. Engaging. Trustworthy.'

# Render animation
bpy.ops.render.render(animation=True)
