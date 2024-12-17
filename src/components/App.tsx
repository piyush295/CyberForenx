import React from 'react';

const App: React.FC = () => {
  return (
    <div className="App">
      <header className="bg-gray-800 text-white p-4">
        <h1 className="text-2xl font-bold">Welcome to CyberForenx Academy</h1>
      </header>
      <main className="p-4">
        <h2 className="text-xl">Your gateway to mastering cybersecurity</h2>
        <p>Explore our courses and enhance your skills!</p>
      </main>
      <footer className="bg-gray-800 text-white p-4 text-center">
        <p>© 2023 CyberForenx Academy. All rights reserved.</p>
      </footer>
    </div>
  );
};

export default App;