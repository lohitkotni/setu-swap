import { NextResponse } from "next/server";
import { getDatabase } from "../../lib/database";

export async function GET(request: Request) {
  try {
    const url = new URL(request.url);
    const escrowAddress = url.searchParams.get("escrowAddress");
    const hashlock = url.searchParams.get("hashlock");
    const chainIdParam = url.searchParams.get("chainId");
    const chainId = chainIdParam ? parseInt(chainIdParam) : undefined;

    const db = await getDatabase();
    let events = [];

    if (escrowAddress && chainId !== undefined) {
      events = await db.getEventsForEscrow(escrowAddress, chainId);
    } else if (hashlock && chainId !== undefined) {
      events = await db.getEventsByHashlock(hashlock, chainId);
    } else {
      events = await db.getAllEvents();
    }

    return NextResponse.json({
      success: true,
      data: events,
      count: events.length,
    });
  } catch (error) {
    console.error("Error fetching events:", error);
    return NextResponse.json(
      { success: false, error: "Failed to fetch events" },
      { status: 500 }
    );
  }
}
