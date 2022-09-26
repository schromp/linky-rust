import "../App.css"


//TODO make more beautiful with card and add go back button
function Newlink(props) {
    return (
        <div className="form-control">
            <h1> Your created Link:</h1>
            <div className="input-group">
                <input type="text" id="link" value={props.created} className="input input-bordered w-full" readOnly/>
                <button className="btn btn-square" onClick={() => {navigator.clipboard.writeText(props.created)}}> Copy </button>
            </div>
            <div className="form-control mt-6">
            <button className="btn btn-primary" onClick={props.statechange}>
              Create new link
            </button>
          </div>
        </div>
    )
}

export default Newlink;