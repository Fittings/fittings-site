import React, {Component} from 'react';

import './App.css';
import Header from './standard/header.js';


class HomeImages extends Component {
    render() {
        return (

            <div className="tabs four">
            <h1>Home</h1>
                <input id="tabC-1" type='radio' name='tabGroupC' defaultChecked></input>
                <input id="tabC-2" type='radio' name='tabGroupC'></input>
                <input id="tabC-3" type='radio' name='tabGroupC'></input>
                <input id="tabC-4" type='radio' name='tabGroupC'></input>
                <div className='row'>
                    <div>
                        <img src={require('./static/home/2012-10-29-hor1.jpg')} className="hor1" alt="hor1"/>
                    </div>
                    <div>
                        <img src={require('./static/home/2015-10-29-wellesbourne.jpg')} className="forth" alt="forth"/>
                    </div>
                    <div>
                        <img src={require('./static/home/2016-10-30-lidl1.jpg')} className="wellesbourne" alt="wellesbourne"/>
                    </div>
                    <div>
                        <img src={require('./static/home/2016-10-30-lidl2.jpg')} className="lidl1" alt="lidl1"/>
                    </div>
                </div>
                <label htmlFor="tabC-1"><img src={require('./static/home/2012-10-29-hor1.jpg')} className="hor1" alt="hor1"/></label>
                <label htmlFor="tabC-2"><img src={require('./static/home/2015-10-29-wellesbourne.jpg')} className="forth" alt="forth"/></label>
                <label htmlFor="tabC-3"><img src={require('./static/home/2016-10-30-lidl1.jpg')} className="wellesbourne" alt="wellesbourne"/></label>
                <label htmlFor="tabC-4"><img src={require('./static/home/2016-10-30-lidl2.jpg')} className="lidl1" alt="lidl1"/></label>

            </div>
        );
    }
}

class App extends Component {
    render() {
        return (

            <div>
                <Header />
                <section className="flex five">
                    <aside className="full fifth-1000"></aside>
                    <article className="full four-fifth-1000">
                        <HomeImages/>
                        <HomeImages/>
                        <HomeImages/>
                        <HomeImages/>
                        <HomeImages/>

                    </article>
                </section>

            </div>
        );
    }
}

export default App;
