# CyberForenx Academy Backend

This is the backend for the CyberForenx Academy project, which provides a comprehensive platform for cybersecurity education.

## Overview

The backend is built using Node.js and TypeScript, utilizing Express for handling HTTP requests and responses. It connects to a database to manage user data, course information, and other relevant data for the application.

## Project Structure

- **src/**: Contains the source code for the backend application.
  - **controllers/**: Contains controller functions that handle incoming requests and responses.
  - **models/**: Defines the data models used in the application.
  - **routes/**: Exports the route definitions for the application.
  - **services/**: Contains business logic and service functions.
  - **app.ts**: The entry point of the backend application.

- **package.json**: Configuration file for npm, listing dependencies and scripts.

- **tsconfig.json**: TypeScript configuration file specifying compiler options.

## Getting Started

1. **Clone the repository**:
   ```
   git clone https://github.com/yourusername/cyberforenx-academy.git
   ```

2. **Navigate to the backend directory**:
   ```
   cd cyberforenx-academy/backend
   ```

3. **Install dependencies**:
   ```
   npm install
   ```

4. **Run the application**:
   ```
   npm start
   ```

## API Documentation

Refer to the `routes/index.ts` file for the list of available API endpoints and their usage.

## Contributing

Contributions are welcome! Please submit a pull request or open an issue for any enhancements or bug fixes.

## License

This project is licensed under the MIT License. See the LICENSE file for details.