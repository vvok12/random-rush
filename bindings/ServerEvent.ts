// This file was generated by [ts-rs](https://github.com/Aleph-Alpha/ts-rs). Do not edit this file manually.
import type { Board } from "./Board";
import type { PlayroomId } from "./PlayroomId";
import type { UserHand } from "./UserHand";
import type { UserId } from "./UserId";
import type { UserMove } from "./UserMove";

export type ServerEvent = { "SendUserId": UserId } | { "SendPlayroomId": PlayroomId } | { "SendBoard": Board } | { "SendUserHand": UserHand } | { "ConfirmMove": UserMove } | { "Error": string };
