import "../App.css";

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
              className="input input-bordered"
            />
          </label>
        </div>

        <div class="alert alert-error shadow-lg">
          <div>
            <svg
              xmlns="http://www.w3.org/2000/svg"
              class="stroke-current flex-shrink-0 h-6 w-6"
              fill="none"
              viewBox="0 0 24 24"
            >
              <path
                stroke-linecap="round"
                stroke-linejoin="round"
                stroke-width="2"
                d="M10 14l2-2m0 0l2-2m-2 2l-2-2m2 2l2 2m7-2a9 9 0 11-18 0 9 9 0 0118 0z"
              />
            </svg>
            <span>You can only use Characters a-z 0-9.</span>
          </div>
        </div>

        <div className="form-control mt-6">
          <button className="btn btn-primary">Generate</button>
        </div>
      </div>
    </div>
  );
}

export default Linkform;
