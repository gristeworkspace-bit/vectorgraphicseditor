import ReactDOM from 'react-dom/client';
import App from './App';
import './styles/index.css';

// Note: StrictMode is disabled for Wasm compatibility
// React 18's StrictMode double-invokes useEffect which can cause issues with Wasm memory
ReactDOM.createRoot(document.getElementById('root')!).render(
    <App />
);
