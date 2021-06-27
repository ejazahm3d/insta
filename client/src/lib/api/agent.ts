import axios, { AxiosResponse } from 'axios';
import { Account } from './account';
import { Comments } from './comment';
import { Followers, Leaders } from './followers-leaders';
import { Posts } from './post';
import { Profiles } from './profile';

axios.defaults.baseURL = 'http://localhost:5000/api';
axios.defaults.withCredentials = true;

const responseBody = <T>(response: AxiosResponse<T>): T => response.data;

export const requests = {
	get: <T>(url: string): Promise<T> => axios.get<T>(url).then(responseBody),
	post: <T, R>(url: string, body: T): Promise<R> => axios.post<R>(url, body).then(responseBody),
	put: <T, R>(url: string, body: T): Promise<R> => axios.put<R>(url, body).then(responseBody),
	delete: <T>(url: string): Promise<T> => axios.delete<T>(url).then(responseBody)
};

export default { Posts, Account, Comments, Profiles, Followers, Leaders };
