import React, { Component } from 'react'
import { Router, Route, Link, IndexRoute, hashHistory, browserHistory } from 'react-router'

class App extends Component {
  render() {
    return (
        <Router history={browserHistory}>
          <Route path='/' component={Home}>
            <Route path='cameron-milsom' component={CameronMilsom} />
            <Route path="*" component={NotFound} />
          </Route>
        </Router>)
  }
}

const Home = () => <h1>This is my home page..?</h1>
const CameronMilsom = () => <span>Cameron Milsom</span>
const NotFound = () => (<div><h1>Not Found!</h1>
                        <span>This page doesn't exist. Maybe it should exist?</span>
                        <span>Vist my </span>
                        <a href="https://github.com/Fittings/fittings-site">GitHub</a>
                        <span> and raise an issue.</span>
                        </div>)

export default App
