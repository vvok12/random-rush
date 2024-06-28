// This file was generated by [ts-rs](https://github.com/Aleph-Alpha/ts-rs). Do not edit this file manually.
import type { BoardId } from "./BoardId";
import type { UserId } from "./UserId";
import type { UserMove } from "./UserMove";

export type ClientEvent = "ReciveUserId" | { "LoadBoard": BoardId } | { "LoadHand": UserId } | { "MakeMove": UserMove };
