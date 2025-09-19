export interface User {
  uuid: string;
  username: string;
  descriptions: string | null;
  profile_picture: string | null;
  keys: Keys;
  pending_friend_requests: string[];
  friend_requests: string[];
  friends: string[];
}

export interface Keys {
  ik_pub: Uint8Array[];
  spk_pub: Uint8Array[];
  opk_pub: Uint8Array[][];
}