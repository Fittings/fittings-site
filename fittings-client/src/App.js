import React, {Component} from 'react';

import './App.css';
import Header from './standard/header.js';
import Gallery from './picture-view/gallery.js';

import image_collection from "./picture-view/images";


class HomeGallery extends Component {
  render() {

    var galleries = [];
    var count=0;
    image_collection.forEach(images =>
    {
      console.log("pushing a gallery");
      galleries.push(<Gallery images={images}/>)
count++;
    });

    return (<li key={count}>{galleries}</li>)
    // return (<div>{galleries}</div>);
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
                    <HomeGallery />

                    </article>
                </section>

            </div>
        );
    }
}

export default App;
