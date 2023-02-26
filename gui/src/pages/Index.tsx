import { Link } from "react-router-dom";

function Index () {
    return (
        <div className="text-center pt-10">
            <h1 className="text-3xl">Lakeshire</h1>
            <ul className="list-none pt-10">
                <li>
                    <Link className="text-lg text-center" to={`/editor`}>
                        <button className="rounded-full bg-orange-400 px-7 py-3 mt-3">Editor</button>
                    </Link>
                    <br/>
                    <Link className="text-lg text-center" to={`/settings`}>
                        <button className="rounded-full bg-orange-400 px-7 py-3 mt-3">Settings</button>
                    </Link>
                </li>
            </ul>
        </div>
    );
}

export default Index;
