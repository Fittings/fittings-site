import React, {Component} from 'react';

class Gallery extends Component {
    render() {
        return (
            <nav className="Header">
                <a href="#" className="brand">
                    <img src={logo} className="logo" alt="logo"/>
                    <span>Fittings</span>
                </a>

                <input id="bmenub" type="checkbox" className="show"></input>
                <label htmlFor="bmenub" className="burger pseudo button">&#9776;</label>

                <div className="menu">
                    <a href="https://github.com/fittings" className="pseudo button icon-github">GitHub</a>
                </div>
            </nav>
        );

    }
}
export default Gallery;
