const postbtn = document.getElementById('post-btn')

const sendData = () => {
    axios.post(
        'http://localhost:8080',
        {
            shortlink: "yup",
            longlink: "http://koziollek.com"
        }
    )
}