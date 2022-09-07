import "../App.css";
import Alert from "./Alert";


//This has to be reworked. shortlink is scaling wrong
function Linkform() {
  return (
    <div className="card flex-shrink-0 w-full max-w-sm shadow-2xl bg-base-100">
      <div className="card-body">
        <div className="form-control">
          <label className="label">
            <span className="label-text">Your very long link</span>
          </label>
          <input
            type="text"
            placeholder="https://verylonglink.com/very/long/link"
            className="input input-bordered"
          />

          <label className="label">
            <span className="label-text">
              Personalized Link: Leave empty to autogenerate
            </span>
          </label>
          <label className="input-group">
            <span>schro.it/</span>
            <input
              type="text"
              placeholder="short link"
              className="input input-bordered w-full"
            />
          </label>
        </div>

        <Alert/>

        <div className="form-control mt-6">
          <button className="btn btn-primary">Generate</button>
        </div>
      </div>
    </div>
  );
}

export default Linkform;
