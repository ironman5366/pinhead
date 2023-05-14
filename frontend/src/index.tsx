import React from 'react';
import ReactDOM from 'react-dom/client';
import {QueryClient, QueryClientProvider} from "react-query";
import {MantineProvider} from "@mantine/core";
import {BrowserRouter} from "react-router-dom";
import App from './App';
import reportWebVitals from './reportWebVitals';
import {ClientProvider} from "./contexts/ClientContext";
import Theme from "./theme";
import './index.css'
import {TokenProvider} from "./contexts/TokenContext";
import {UserProvider} from "./contexts/UserContext";
import './i18n'

const root = ReactDOM.createRoot(
  document.getElementById('root') as HTMLElement
);

const queryClient = new QueryClient()

root.render(
  <React.StrictMode>
      <MantineProvider withGlobalStyles withNormalizeCSS theme={Theme}>
          <QueryClientProvider client={queryClient}>
              <BrowserRouter>
                  <TokenProvider>
                      <ClientProvider>
                              <UserProvider>
                                  <App />
                              </UserProvider>
                      </ClientProvider>
                  </TokenProvider>
              </BrowserRouter>
          </QueryClientProvider>
      </MantineProvider>

  </React.StrictMode>
);

// If you want to start measuring performance in your app, pass a function
// to log results (for example: reportWebVitals(console.log))
// or send to an analytics endpoint. Learn more: https://bit.ly/CRA-vitals
reportWebVitals();
