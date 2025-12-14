from PIL import Image

# Create a reference image (100x100 red square)
ref = Image.new('RGB', (100, 100), color='red')
ref.save('test_assets/ref.png')

# Create an identical implementation
impl_identical = Image.new('RGB', (100, 100), color='red')
impl_identical.save('test_assets/impl_identical.png')

# Create a slightly different implementation (blue instead of red)
impl_different = Image.new('RGB', (100, 100), color='blue')
impl_different.save('test_assets/impl_different.png')

print("Created test images in test_assets/")
