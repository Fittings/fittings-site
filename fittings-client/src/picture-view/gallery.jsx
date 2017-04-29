import React, {Component} from 'react';



class Gallery extends Component {
  constructor(props) {
    super(props);


    // This binding is necessary to make `this` work in the callback
    this.handleClick = this.handleClick.bind(this);
  }

 handleClick() {
    // e.preventDefault();
    console.log('The link was clicked.');
  }

    render() {
        return (
            <div>
                <h1>{this.props.images.name}</h1>
                <div className="flex two">
                    {this.props.images.images.map((url, index) => {
                        var img = process.env.PUBLIC_URL + url.url;
                        return (
                            <div>
                                <article className="card" onClick="showFullImage">
                                    <img src={img} className="stack" alt={img}/>
                                    <button className="button stack" onClick={this.handleClick()}>Full size</button>
                                </article>
                            </div>
                        )
                    })}
                </div>
            </div>
        )
    }
}
export default Gallery;
