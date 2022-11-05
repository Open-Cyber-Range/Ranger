
import React from 'react';
import ReactDOM from 'react-dom/client';
import './index.css';
import {Provider} from 'react-redux';
import App from './App';
import reportWebVitals from './reportWebVitals';
import store from './store';

// eslint-disable-next-line import/no-unassigned-import
import './i18n';

const root = ReactDOM.createRoot(
  document.querySelector('#root')!,
);
root.render(
  <React.StrictMode>
    <Provider store={store}>
      <App/>
    </Provider>
  </React.StrictMode>,
);

reportWebVitals();
