#TACIT-PY: resolve_module('PIL', 'Pillow')
from PIL import Image
import requests
from io import BytesIO

response = requests.get('https://www.python.org/static/community_logos/python-logo-master-v3-TM-flattened.png')
img = Image.open(BytesIO(response.content))
print(img)
