import React from 'react';
import { renderToString, renderToStaticMarkup } from 'react-dom/server';
import App from './App';
import './index.css';

export const Index = (params: string | undefined) => {
  const props = params ? JSON.parse(params) : {};
  const app = renderToString(<App {...props} />);

  return `<!doctype html>
  <html>
    <head>
      <title>React RUST SSR</title>
      <link rel="stylesheet" href="./styles/ssr.css">
      </head>
      <body>
      <div id="root">${app}</div>
      </body>
      
      ${renderToStaticMarkup(
    <script
      dangerouslySetInnerHTML={{
        __html: `window.__INITIAL_PROPS__ = ${params || null};`,
      }}
    />
  )}
      <script async defer src="./scripts/bundle.js"></script>
  </html>`;
};
