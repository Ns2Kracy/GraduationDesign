import axios from "axios";

const instance = axios.create({
	baseURL: "http://localhost:7590",
	headers: {
		"Content-Type": "application/json",
	},
});

instance.interceptors.request.use(
	(config) => {
		return config;
	},
	(error) => {
		return Promise.reject(error);
	},
);

instance.interceptors.response.use(
	(response) => {
		return response;
	},
	(error) => {
		return {
			code: error.response.status,
			message: error.message,
		};
	},
);

instance.defaults.headers.common["Authorization"] =
	localStorage.getItem("token") || "";

export const setToken = (token?: string) => {
	instance.defaults.headers.common["Authorization"] = token ?? "";
	localStorage.setItem("token", token ?? "");
};

export default instance;
