import React, {Component} from 'react';
import logo from './logo.svg';

import hor1 from './static/home/2012-10-29-hor1.jpg';
import forth from './static/home/2013-08-25-forth.jpg';
import wellesbourne from './static/home/2015-10-29-wellesbourne.jpg';
import lidl1 from './static/home/2016-10-30-lidl1.jpg';
import lidl2 from './static/home/2016-10-30-lidl2.jpg';


import './App.css';
//
// const Header = () =>

class App extends Component {
    render() {
        return (
            <div>
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



                {/* <nav className="header">
                    <a className="brand" href="/">
                        <img src={logo} className="logo" alt="logo"/>
                        <span>Cameron Milsom</span>
                    </a>


                    <input id="bmenub" type="checkbox" className="show"></input>
                    <label for="bmenub" className="burger pseudo button">menu</label>

                    <div className="menu">
                      <a href="#" className="pseudo button icon-picture">Demo</a>
                      <a href="#" className="button icon-puzzle">Plugins</a>
                    </div>

                </nav> */}
                {/* <header id="main-header">

                    <div className="centre, header-centre">
                        <div className="header-clear"></div>
                        <a className="logo" href="/"></a>
                        <h1 className="header-title">Cameron Milsom</h1>
                        <nav className="header-nav">
                            <ul className="hor-nav-list">
                                <li id="github">
                                    <a href="https://www.github.com/fittings">GitHub</a>
                                </li>
                            </ul>
                        </nav>
                    </div>

                </header>

                <main id="content">
                    <div className="page-intro">
                        <div className="overlay">
                            <h1>Welcome</h1>
                        </div>
                    </div>

                    <div id="centre-clear">
                        <div id="chapter">
                            <h1>
                                Lorem Ipsum
                            </h1>
                            <p>
                                Lorem ipsum dolor sit amet, consectetur adipiscing elit. Nulla interdum in nibh eu feugiat. Nullam ultricies efficitur mauris. Sed vulputate tortor nisi, quis eleifend lorem tempus nec. Curabitur suscipit ut nulla quis blandit. Nullam bibendum tincidunt quam vitae rutrum. Ut porttitor, diam aliquet dignissim fringilla, neque urna hendrerit massa, nec aliquet justo urna eget mauris. Ut ullamcorper posuere erat vel ultrices. Fusce fermentum, mauris sit amet varius vestibulum, tortor tortor dapibus lectus, vitae euismod leo elit quis ex. Integer vehicula fringilla metus, hendrerit sagittis ante rutrum id. In mollis at orci id tincidunt. Nulla fringilla urna eget lectus malesuada porttitor. Ut arcu ligula, iaculis et odio in, rhoncus fermentum lacus. In consectetur lectus nec ex varius fringilla ut vel diam. Phasellus et sem vitae purus molestie sodales vel ac est.
                            </p>
                            <p>
                                Proin venenatis, est tincidunt porta ornare, metus dolor tristique massa, ac blandit neque libero in libero. Praesent finibus ut ligula id convallis. Curabitur eu congue ipsum, quis euismod neque. Sed mattis nisi nisl, ut hendrerit sem vulputate eu. Suspendisse tristique purus ut ligula egestas, sit amet bibendum magna iaculis. Sed vitae imperdiet leo. Ut tincidunt ex non mi tincidunt semper. Nulla iaculis venenatis ex, in condimentum ligula pharetra in. Pellentesque rutrum ut erat ac lobortis. Cras egestas felis augue, at tristique velit feugiat malesuada. Maecenas ex est, dapibus nec fringilla et, convallis in metus.
                            </p>
                            <p>
                                Aliquam erat volutpat. Aliquam tincidunt est lacus, eget tempor mauris fringilla vel. Maecenas id accumsan tortor. Mauris quis eros ac nulla aliquet malesuada id sit amet ex. Curabitur sed risus quis turpis finibus luctus nec eu sem. Interdum et malesuada fames ac ante ipsum primis in faucibus. Mauris ut semper nibh. Quisque sed augue aliquam, porttitor risus eu, tempus sapien. Curabitur placerat ultrices placerat. Duis ut egestas ante.
                            </p>
                            <p>
                                Nam eu commodo odio. Interdum et malesuada fames ac ante ipsum primis in faucibus. Sed consequat urna sed ligula euismod, nec porttitor nibh mollis. Nullam luctus orci in ultrices mollis. Pellentesque in molestie lacus. Pellentesque ut malesuada mi. Donec elementum arcu ipsum, sit amet dapibus orci blandit ac.
                            </p>
                            <p>
                                Nullam turpis ligula, rutrum non blandit non, eleifend et orci. Cras feugiat libero vitae ante ultricies, eu placerat erat lobortis. Praesent quis nisi nec diam fermentum sagittis. Quisque mi urna, tincidunt at ipsum ut, dignissim egestas dui. Donec mollis a nunc a pretium. Sed gravida elementum magna sit amet gravida. Proin dignissim orci laoreet congue mattis. Vestibulum tristique venenatis justo sed viverra. Pellentesque luctus convallis dui nec rutrum. Sed elementum nibh non molestie viverra. Nunc malesuada, dui ac blandit sodales, elit purus vestibulum metus, nec egestas tellus orci vitae enim.
                            </p>
                        </div>
                    </div>
                </main>

                <footer id="main-footer">
                    <div className="footer-clearing"></div>
                    <div className="footer-aside">
                        <p>Cameron Milsom</p>
                    </div>
                </footer> */}

            </div>
        );
    }
}

export default App;
