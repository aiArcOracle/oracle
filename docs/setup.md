# SETUP

## PREREQUISITES

- **RUST & CARGO**: Install Rust via `rustup` (`curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y`).
- **GIT**: Ensure Git is installed (`git --version`).
- **TOGETHER AI API KEY**: Obtain from [Together AI](https://www.together.ai).
- **PYTHON 3, PIP, AND MKDOCS**: Install via `sudo apt-get install python3 python3-pip` (Ubuntu) and ensure `pip` permissions allow installations.

## INSTALLATION

You can set up the ArcOracle documentation independently or within an existing ArcOracle project. Choose one of the following options:

### OPTION 1: SET UP DOCUMENTATION INDEPENDENTLY
   ```bash
   mkdir arcoracle-docs
   cd arcoracle-docs
   cp -r backend/docs/* .
   mkdocs serve
   ```

### OPTION 2: SET UP DOCUMENTATION WITHIN ARCORACLE PROJECT
   ```bash
   cd path/to/arcoracle/backend/docs
   mkdocs serve
   ```

## RUNNING THE DOCUMENTATION LOCALLY

1. Navigate to the docs directory:
   ```bash
   cd backend/docs
   ```

2. Serve the documentation locally:
   ```bash
   mkdocs serve
   ```
   - Open `http://localhost:8000` in your browser to preview the site.

## DEPLOYING TO GITHUB PAGES

1. Ensure you have a GitHub repository (`https://github.com/aiArcOracle/oracle`).
2. Configure `mkdocs.yml` (included in this directory) with your repository URL and enable GitHub Pages:
   ```yaml
   site_url: https://aiarcoracle.github.io/oracle/
   repo_name: aiArcOracle/oracle
   repo_url: https://github.com/aiArcOracle/oracle
   ```
3. Deploy the site:
   ```bash
   mkdocs gh-deploy
   ```
   - This will publish the documentation to `https://aiarcoracle.github.io/oracle/`.

## TROUBLESHOOTING

- **PEP 668 RESTRICTIONS**: If `pip install mkdocs mkdocs-material` fails due to PEP 668, use one of these approaches:
  1. **VIRTUAL ENVIRONMENT (RECOMMENDED)**:
     ```bash
     python3 -m venv venv
     source venv/bin/activate  # On Windows, use 'venv\\Scripts\\activate'
     pip install mkdocs mkdocs-material
     ```
  2. **USER-LEVEL INSTALLATION**:
     ```bash
     pip install --user mkdocs mkdocs-material
     ```
  3. **OVERRIDE SYSTEM RESTRICTIONS (NOT RECOMMENDED)**:
     ```bash
     pip install --break-system-packages mkdocs mkdocs-material
     ```
  - After resolving, proceed with `mkdocs serve` or `mkdocs gh-deploy`.

- **GITHUB PAGES ISSUES**: Verify repository settings, branch permissions, and `mkdocs.yml` configuration.
