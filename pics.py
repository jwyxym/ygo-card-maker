from PIL import Image
from pathlib import Path

def is_square_image(image_path, output_path, code):
    """检查图片是否为正方形"""
    try:
        with Image.open(image_path) as img:
            width, height = img.size
            if width != height:
                cropped_img = img.crop((0, 0, width, width))
                cropped_img.save(output_path)
    except Exception as e:
        print(f"无法读取图片 {image_path}: {e}")
        return None
    
path = Path('./')

for item in path.iterdir():
    if item.is_file() and item.name.endswith('.jpg'):
        row = is_square_image(item.name, item.name, item.name.replace('.jpg', ''))