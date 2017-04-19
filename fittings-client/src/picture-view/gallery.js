import React, {Component} from 'react';


function ActionLink() {
  function handleClick(e) {
    e.preventDefault();
    console.log('The link was clicked.');
  }}


class Gallery extends Component {
    render() {
        return (
            <div>
                <h1>{this.props.images.name}</h1>
                <div className="flex two">
                    {this.props.images.images.map((url, index) => {
                        console.log("Cameron! " + index)
                        var img = process.env.PUBLIC_URL + url.url;
                        return (
                            <div>
                                <article className="card" onClick="showFullImage"><img src={img} className={img} alt={img}/></article>
                            </div>
                        )
                    })}
                </div>
            </div>
        )
    }
}
export default Gallery;
