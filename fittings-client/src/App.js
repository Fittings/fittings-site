import React, {Component} from 'react';

import './App.css';
import Header from './standard/header.js';
import Gallery from './picture-view/gallery.jsx';

import image_collection from "./picture-view/images";

class HomeGallery extends Component {
    render() {
        return (
            <div>
                {image_collection.map((images, index) => {
                    return <Gallery images={images}/>
                })}
            </div>
        )
    }
}

class GeneralAside extends Component {
    render() {
        return (
            <aside className="full fifth-1000">
                <h1>Articles</h1>
                <div className="links flex two three-500 five-800 one-1000">
                    <div>
                        <a className="pseudo button" href="https://google.com">Google</a>
                    </div>
                    <div>
                        <a className="pseudo button" href="#Netherlands">Netherlands</a>
                    </div>
                    <div>
                        <a className="pseudo button" href="#Spain">Spain</a>
                    </div>
                </div>
            </aside>
        )
    }

}

class App extends Component {
    render() {
        return (
            <div>
                <Header/>
                <section className="flex five">
                    <GeneralAside/>

                    <article className="full four-fifth-1000">
                        <HomeGallery/>
                    </article>
                </section>

            </div>
        );
    }
}

export default App;
