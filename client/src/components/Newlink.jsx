import "../App.css"

function Newlink(props) {
    return (
        <div className="form-control">
            <div className="input-group">
                <input type="text" id="link" value={props.created} className="input input-bordered w-full" readOnly/>
                <button className="btn btn-square" onClick={() => {navigator.clipboard.writeText(props.created)}}> Copy </button>
            </div>
        </div>
    )
}

export default Newlink;