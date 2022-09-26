import "../App.css";
import Alert from "./Alert";
import Newlink from "./Newlink";
import Axios from "axios";
import { useState } from "react";
import { hasHttp, isValidLink, isValidShortlink } from "../utilities/utils";

//TODO change to .env
const baseUrl = "http://127.0.0.1:8080/";

function Linkform() {
  const [shortlink, setShortlink] = useState("");
  const [longlink, setLonglink] = useState("");

  const [slError, setSlError] = useState(false);
  const [llError, setLlError] = useState(false);
  const [inputError, setInputError] = useState(false);
  const [generalError, setGeneralError] = useState(false);

  const [gotLink, setGotLink] = useState(false);

  const handleShortlink = (event) => {
    if (isValidShortlink(event.target.value)) {
      setSlError(false)
      setShortlink(event.target.value)
    } else {
      setSlError(true)
      setShortlink(null)
    }
  }

  const handleLonglink = (event) => {

    if (isValidLink(event.target.value)) {
      setLlError(false)
      setLonglink(event.target.value)
    } else {
      setLlError(true)
      setLonglink(null)
    }
  }

  const createLink = () => {

    if (!llError && !slError) {
      Axios.post(baseUrl + "createLink", {
        shortlink: shortlink,
        longlink: hasHttp(longlink) ? longlink : ("http://" + longlink),
      })
        .then(function (response) {
          setGotLink(baseUrl + response.data.shortlink)
        })
        .catch(function (error) { 
          if(error.response.status === 400) {
            setInputError(true)
          } else {
            setGeneralError(true)
          }
         })
    }
  }

  const reset = () => {
    setGotLink(false)
    setLlError(false)
    setSlError(false)
    setInputError(false)
    setGeneralError(false)
    setLonglink("")
    setShortlink("")
  }

  const handleKeyDown = (event) => {
    if (event.key === 'Enter') {
      createLink()
    }
  }

  if (!gotLink) {
    return (
      <div className="card flex-shrink-0 w-full max-w-sm shadow-2xl bg-base-100">
        <div className="card-body">
          <div className="form-control" onSubmit={createLink}>
            <label className="label">
              <span className="label-text">Your very long link</span>
            </label>
            <input
              type="text"
              placeholder="https://verylonglink.com/very/long/link"
              className="input input-bordered"
              onChange={handleLonglink}
              onKeyDown={handleKeyDown}
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
                placeholder="shortlink"
                className="input input-bordered w-full"
                onChange={handleShortlink}
                onKeyDown={handleKeyDown}
              />
            </label>

            <div>
              {llError ? <Alert message="Please input a valid link" /> : <p />}
              {slError ? <Alert message="You can only use charcters a-z and 0-1" /> : <p />}
              {inputError ? <Alert message="Invalid Link" /> : <p />}
              {generalError ? <Alert message="An error occured" /> : <p />}
            </div>

            <button type="submit" className="btn btn-primary mt-4" onClick={createLink}>
              Generate
            </button>
          </div>
        </div>
      </div>
    );
  } else {
    return <Newlink created={gotLink} statechange={reset} />
  }

}

export default Linkform;
