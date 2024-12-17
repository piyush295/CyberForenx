# CyberForenx Academy Frontend

Welcome to the CyberForenx Academy frontend documentation. This document provides an overview of the frontend application, its structure, and how to get started.

## Project Structure

The frontend application is built using React and TypeScript. Below is the structure of the frontend directory:

```
frontend/
├── public/
│   └── index.html         # Main HTML file for the application
├── src/
│   ├── components/        # Contains React components
│   │   └── App.tsx       # Main App component
│   ├── styles/            # Contains CSS styles
│   │   └── index.css      # Main stylesheet
│   └── index.tsx          # Entry point for the React application
├── package.json            # NPM configuration file
└── tsconfig.json           # TypeScript configuration file
```

## Getting Started

To get started with the frontend application, follow these steps:

1. **Clone the Repository**: 
   ```bash
   git clone <repository-url>
   cd cyberforenx-academy/frontend
   ```

2. **Install Dependencies**: 
   ```bash
   npm install
   ```

3. **Run the Application**: 
   ```bash
   npm start
   ```

   This will start the development server and open the application in your default web browser.

## Development

- The main component of the application is located in `src/components/App.tsx`.
- Styles can be modified in `src/styles/index.css`.
- The entry point for the React application is `src/index.tsx`.

## Building for Production

To create a production build of the application, run:

```bash
npm run build
```

This will generate a `build` directory containing the optimized application.

## Contributing

Contributions are welcome! Please submit a pull request or open an issue for any enhancements or bug fixes.

## License

This project is licensed under the MIT License. See the LICENSE file for more details.