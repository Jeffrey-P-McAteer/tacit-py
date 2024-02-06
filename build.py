
import os
import sys
import subprocess
import shutil

def main():

  if not shutil.which('python'):
    print('Refusing to continue without python installed!')
    sys.exit(1)

  if not shutil.which('cargo'):
    print('Refusing to continue without cargo installed!')
    sys.exit(1)

  # Ensure we have pyoxidizer (alternatively, consider https://pyoxidizer.readthedocs.io/en/stable/pyoxidizer_getting_started.html#id5 )
  pyoxidizer_site_packages = os.path.join(os.path.dirname(__file__), 'target', 'pyoxidizer-site-packages')
  if not shutil.which('pyoxidizer'):
    print('Using pip to install pyoxidizer...')
    os.makedirs(pyoxidizer_site_packages, exist_ok=True)
    subprocess.run([
      sys.executable, '-m', 'pip', 'install', f'--target={pyoxidizer_site_packages}', 'pyoxidizer'
    ])

  if not shutil.which('pyoxidizer'):
    pyoxidizer_site_packages_bin = os.path.join(pyoxidizer_site_packages, 'bin')
    print('Please add the following directory to your PATH:')
    print(f'   {pyoxidizer_site_packages_bin}')
    print('For example, by running:')
    if os.name == 'nt':
      print(f'   set PATH={pyoxidizer_site_packages_bin};%PATH%')
    else:
      print(f'   export PATH={pyoxidizer_site_packages_bin}:$PATH')
    print(f'')
    sys.exit(1)

  print('pyoxidizer = ', shutil.which('pyoxidizer'))

  pyembedded_dir = os.path.join(os.path.dirname(__file__), 'pyembedded')
  if not os.path.exists(pyembedded_dir):
    subprocess.run('pyoxidizer generate-python-embedding-artifacts'.split() + [ pyembedded_dir ])

  # Perform the build
  os.environ['PYO3_CONFIG_FILE'] = os.path.join(pyembedded_dir, 'pyo3-build-config-file.txt')

  subprocess.run('cargo build --release'.split())

  print()
  print('Done building!')
  print()

  if not os.name == 'nt' and shutil.which('ldd'):
    print('target/release/tacit-py')
    subprocess.run('ldd target/release/tacit-py'.split())

  print()
  print('Running..')
  subprocess.run('cargo run --release'.split())




if __name__ == '__main__':
  main()

