import React, { Component } from 'react'
import { Router, Route, Link, IndexRoute, hashHistory, browserHistory } from 'react-router'

class App extends Component {
  render() {
    return (
        <Router history={hashHistory}>
          <Route path='/' component={Home} />
          <Route path='/cameron-milsom' component={CameronMilsom} />
        </Router>)
  }
}

const Home = () => <h1>This is my home page...</h1>
const CameronMilsom = () => <span>Cameron Milsom</span>

export default App
