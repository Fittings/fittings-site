// /* global window document */
//
// import React from 'react';
// import { render } from 'react-dom';
// import { BrowserRouter as Router } from 'react-router-dom';
//
//
// const AppClient = () => (
//   <Router>
//     <App />
//   </Router>
// );
//
//
// ReactDOM.render(<App />, document.getElementById('root');
// // window.onload = () => {
// //   render(<h1>Hello Cameron</h1>);
// // };
import React from 'react'
import ReactDOM from 'react-dom'
import App from './components/app'
import Header from './components/standard/header'

// ReactDOM.render(<Header />, document.getElementById('body'))
ReactDOM.render(<App />, document.getElementById('root'))
