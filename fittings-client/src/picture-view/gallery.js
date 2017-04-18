import React, {Component} from 'react';

class Gallery extends Component {
    render() {

        console.log("Rendering a gallery");
        var inputs = [];
        var rows = [];
        var labels = [];

        console.log(this.props.images);
        var count = 0;
        this.props.images.images.forEach((url, index) => {
            if (index < 4) {
                // console.log("url: " + url.url);
                var img = process.env.PUBLIC_URL + url.url;
                inputs.push(
                    <input id={img + "_" + count} type='radio' name='tabGroupC'></input>
                );
                rows.push(
                    <div><img src={img} className={img + "_" + count} alt={img + "_" + count}/></div>
                );
                labels.push(
                    <label htmlFor={img + "_" + count}><img src={url.url} className={img + "_" + count} alt={img + "_" + count}/></label>
                );
                count++;
            }

        });

        return (
            
            <div className="tabs four">
                <h1>{this.props.images.name}</h1>
                {inputs}
                <div className='row'>
                    {rows}
                </div>
                {labels}
            </div>
        );
    }
}
export default Gallery;
